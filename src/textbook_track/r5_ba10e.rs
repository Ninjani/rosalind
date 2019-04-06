use crate::textbook_track::hidden_markov_models::{HMMError, HMM};
use crate::utils;
use failure::Error;
use hashbrown::HashMap;
use itertools::Itertools;
use ndarray::{Array3, Axis};

#[derive(Debug)]
pub struct ProfileHMM {
    threshold: f32,
    alphabet: Vec<char>,
    alphabet_index: HashMap<char, usize>,
    gap_number: usize,
    msa: Vec<Vec<usize>>,
    length: usize,
    seed_alignment: Vec<SeedColumn>,
    num_active: usize,
    transition_matrix: HashMap<(State, State), f32>,
    emission_matrix: Array3<f32>,
    pseudocount: f32,
}

impl ProfileHMM {
    pub fn new(
        threshold: f32,
        pseudocount: Option<f32>,
        alphabet: Vec<char>,
        alphabet_index: HashMap<char, usize>,
        msa_section: &str,
    ) -> Self {
        let mut alphabet_index = alphabet_index;
        let mut alphabet = alphabet;
        alphabet.push('-');
        alphabet_index.insert('-', alphabet.len() - 1);
        let gap_number = alphabet.len() - 1;
        let msa: Vec<Vec<_>> = msa_section
            .split('\n')
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().chars().map(|c| alphabet_index[&c]).collect())
            .collect();
        let length = msa[0].len();
        let mut num_active = 0;
        let seed_alignment = (0..length)
            .map(|i| {
                let num_gaps = msa
                    .iter()
                    .filter(|sequence| sequence[i] == gap_number)
                    .count() as f32;
                let gap_ratio = num_gaps / length as f32;
                if gap_ratio >= threshold {
                    SeedColumn::Inactive
                } else {
                    num_active += 1;
                    SeedColumn::Active
                }
            })
            .collect();
        let len_alphabet = alphabet.len();
        let mut hmm = ProfileHMM {
            threshold,
            pseudocount: match pseudocount {
                Some(p) => p,
                None => 0.,
            },
            alphabet,
            alphabet_index,
            gap_number,
            msa,
            length,
            seed_alignment,
            num_active,
            transition_matrix: HashMap::new(),
            // 0 = match, 1 = delete, 2 = insert
            emission_matrix: Array3::zeros((3, num_active + 1, len_alphabet)),
        };
        let emission_counts = hmm.count_emissions();
        let emission_sums = emission_counts.sum_axis(Axis(2));
        let mut emission_matrix = emission_counts / emission_sums.clone().insert_axis(Axis(2));
        emission_matrix += hmm.pseudocount;
        emission_matrix =
            emission_matrix.clone() / emission_matrix.sum_axis(Axis(2)).insert_axis(Axis(2));
        let transition_counts = hmm.count_transitions();
        println!("{:?}", transition_counts);
        let mut transition_matrix: HashMap<_, _> = transition_counts
            .into_iter()
            .map(|((s1, s2), count)| {
                let count = count + hmm.pseudocount;
                match s1 {
                    State::Start => ((s1, s2), count / hmm.msa.len() as f32),
                    State::Match(i) => ((s1, s2), count / emission_sums[[0, i]]),
                    State::Delete(i) => ((s1, s2), count / emission_sums[[1, i]]),
                    State::Insert(i) => ((s1, s2), count / emission_sums[[2, i]]),
                    State::End => panic!("End as previous state"),
                }
            })
            .collect();
        for i in 0..=hmm.num_active {
            let first = if i == 0 {
                vec![State::Start, State::Insert(0)].into_iter()
            } else {
                vec![State::Match(i), State::Delete(i), State::Insert(i)].into_iter()
            };
            let second = if i == num_active {
                vec![State::Insert(num_active), State::End].into_iter()
            } else {
                vec![State::Insert(i), State::Match(i + 1), State::Delete(i + 1)].into_iter()
            };
            for (s1, s2) in first.cartesian_product(second) {
                *transition_matrix.entry((s1, s2)).or_insert(0.) += hmm.pseudocount;
            }
        }

        hmm.emission_matrix = emission_matrix;
        hmm.transition_matrix = transition_matrix;
        hmm
    }

    fn count_emissions(&self) -> Array3<f32> {
        let mut emission_counts =
            Array3::<f32>::zeros((3, self.num_active + 1, self.alphabet.len()));
        let mut active_j = 0;
        println!("{:?}", self.seed_alignment);
        for j in 0..self.length {
            match self.seed_alignment[j] {
                SeedColumn::Active => {
                    active_j += 1;
                    for sequence in &self.msa {
                        let character = sequence[j];
                        if character != self.gap_number {
                            emission_counts[[0, active_j, character]] += 1.;
                        } else {
                            emission_counts[[1, active_j, character]] += 1.;
                        }
                    }
                }
                SeedColumn::Inactive => {
                    for sequence in &self.msa {
                        let character = sequence[j];
                        if character != self.gap_number {
                            emission_counts[[2, active_j, character]] += 1.;
                        }
                    }
                }
            }
        }
        emission_counts
    }

    fn count_transitions(&self) -> HashMap<(State, State), f32> {
        let mut transition_counts = HashMap::new();
        let (mut previous_state, mut state);
        let mut active_j;
        for sequence in &self.msa {
            previous_state = State::Start;
            active_j = 0;
            for (j, character) in sequence.iter().enumerate() {
                state = match self.seed_alignment[j] {
                    SeedColumn::Active => {
                        active_j += 1;
                        Some({
                            if *character == self.gap_number {
                                State::Delete(active_j)
                            } else {
                                State::Match(active_j)
                            }
                        })
                    }
                    SeedColumn::Inactive => {
                        if *character != self.gap_number {
                            Some(State::Insert(active_j))
                        } else {
                            None
                        }
                    }
                };
                if let Some(state) = state {
                    *transition_counts
                        .entry((previous_state, state))
                        .or_insert(0.) += 1.;
                    previous_state = state;
                }
            }
            *transition_counts
                .entry((previous_state, State::End))
                .or_insert(0.) += 1.;
        }
        transition_counts
    }

    fn get_transition_line(&self, first: State) -> Vec<String> {
        let mut counts = Vec::new();
        counts.push(
            *self
                .transition_matrix
                .get(&(first, State::Start))
                .unwrap_or(&0.),
        );
        counts.push(
            *self
                .transition_matrix
                .get(&(first, State::Insert(0)))
                .unwrap_or(&0.),
        );
        for i in 1..=self.num_active {
            counts.push(
                *self
                    .transition_matrix
                    .get(&(first, State::Match(i)))
                    .unwrap_or(&0.),
            );
            counts.push(
                *self
                    .transition_matrix
                    .get(&(first, State::Delete(i)))
                    .unwrap_or(&0.),
            );
            counts.push(
                *self
                    .transition_matrix
                    .get(&(first, State::Insert(i)))
                    .unwrap_or(&0.),
            );
        }
        counts.push(
            *self
                .transition_matrix
                .get(&(first, State::End))
                .unwrap_or(&0.),
        );
        counts.into_iter().map(|f| f.min(1.).to_string()).collect()
    }

    pub fn print_transitions(&self) {
        for i in 0..=self.num_active {
            if i == 0 {
                print!("S I0 ")
            } else {
                print!("M{} D{} I{} ", i, i, i);
            }
        }
        println!("E");
        for i in 0..=self.num_active {
            if i == 0 {
                println!("S\t{}", self.get_transition_line(State::Start).join("\t"));
                println!(
                    "I0\t{}",
                    self.get_transition_line(State::Insert(0)).join("\t")
                );
            } else {
                println!(
                    "M{}\t{}",
                    i,
                    self.get_transition_line(State::Match(i)).join("\t")
                );
                println!(
                    "D{}\t{}",
                    i,
                    self.get_transition_line(State::Delete(i)).join("\t")
                );
                println!(
                    "I{}\t{}",
                    i,
                    self.get_transition_line(State::Insert(i)).join("\t")
                );
            }
        }
        println!("E\t{}", self.get_transition_line(State::End).join("\t"));
    }

    pub fn print_emissions(&self) {
        fn format_line<T: ToString>(counts: impl Iterator<Item = T>) -> String {
            counts
                .map(|c| c.to_string())
                .map(|s| if s == "NaN" { "0".into() } else { s })
                .collect::<Vec<_>>()
                .join("\t")
        }
        println!(
            "\t{}",
            format_line(self.alphabet.iter().take(self.alphabet.len() - 1))
        );
        println!(
            "S\t{}",
            format_line((0..self.alphabet.len() - 1).map(|_| 0))
        );
        println!(
            "I0\t{}",
            format_line(
                self.emission_matrix
                    .slice(s![0, 0, ..self.alphabet.len() - 1])
                    .iter()
            )
        );
        for i in 1..=self.num_active {
            println!(
                "M{}\t{}",
                i,
                format_line(
                    self.emission_matrix
                        .slice(s![0, i, ..self.alphabet.len() - 1])
                        .iter()
                )
            );
            println!(
                "D{}\t{}",
                i,
                format_line(
                    self.emission_matrix
                        .slice(s![1, i, ..self.alphabet.len() - 1])
                        .iter()
                )
            );
            println!(
                "I{}\t{}",
                i,
                format_line(
                    self.emission_matrix
                        .slice(s![2, i, ..self.alphabet.len() - 1])
                        .iter()
                )
            );
        }
        println!(
            "E\t{}",
            format_line((0..self.alphabet.len() - 1).map(|_| 0))
        );
    }
}

/// Construct a Profile HMM
///
/// Given: A threshold θ, followed by an alphabet Σ,
/// followed by a multiple alignment Alignment whose strings are formed from Σ.
///
/// Return: The transition and emission probabilities of the profile HMM HMM(Alignment, θ).
pub fn rosalind_ba10e() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba10e.txt");
    let mut sections = contents.split("--------");
    let threshold = sections
        .next()
        .ok_or(HMMError::FormatError("Missing threshold".into()))?
        .trim()
        .parse::<f32>()?;
    let (alphabet, alphabet_index) = HMM::read_chars(
        sections
            .next()
            .ok_or(HMMError::FormatError("Missing alphabet".into()))?,
    );
    let msa_section = sections
        .next()
        .ok_or(HMMError::FormatError("Missing alignment".into()))?;
    let hmm = ProfileHMM::new(threshold, None, alphabet, alphabet_index, msa_section);
    hmm.print_transitions();
    println!("--------");
    hmm.print_emissions();
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum State {
    Start,
    Match(usize),
    Insert(usize),
    Delete(usize),
    End,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum SeedColumn {
    Active,
    Inactive,
}
