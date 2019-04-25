use crate::utils;
use failure::Error;
use hashbrown::HashMap;
use ndarray::{Array2, Array3, Axis};

#[derive(Fail, Debug)]
pub enum HMMError {
    #[fail(display = "Wrong input format: {}", _0)]
    InputFormatError(String),
}

/// Hidden Markov Model
pub struct HMM {
    pub alphabet: Vec<char>,
    pub states: Vec<char>,
    pub alphabet_index: HashMap<char, usize>,
    pub state_index: HashMap<char, usize>,
    pub transition_matrix: Array2<f64>,
    pub emission_matrix: Array2<f64>,
}

pub fn read_chars(line: &str) -> (Vec<char>, HashMap<char, usize>) {
    let chars = utils::parse_chars(line);
    let char_index = get_char_index(&chars);
    (chars, char_index)
}

/// list of chars to hashmap of char to index
fn get_char_index(chars: &[char]) -> HashMap<char, usize> {
    chars.iter().enumerate().map(|(i, c)| (*c, i)).collect()
}

/// Read a matrix
/// e.g.
///     A   B
/// A   0.194   0.806
/// B   0.273   0.727
pub fn read_probability_matrix(
    contents: &str,
    row_indices: &HashMap<char, usize>,
    column_indices: &HashMap<char, usize>,
) -> Result<Array2<f64>, Error> {
    let mut matrix_section = contents.trim().split('\n');
    let columns = utils::parse_chars(
        matrix_section
            .next()
            .ok_or_else(|| HMMError::InputFormatError("Missing column names".into()))?,
    );
    let mut matrix = Array2::<f64>::zeros((row_indices.len(), column_indices.len()));
    for row in matrix_section {
        let mut parts = row.trim().split_whitespace();
        let state = parts
            .next()
            .ok_or_else(|| HMMError::InputFormatError("Missing row name".into()))?
            .chars()
            .next()
            .ok_or_else(|| HMMError::InputFormatError("State must be a character".into()))?;
        for (i, probability) in parts.enumerate() {
            matrix[[row_indices[&state], column_indices[&columns[i]]]] =
                probability.parse::<f64>()?;
        }
    }
    Ok(matrix)
}

impl HMM {
    /// Read HMM in format:
    /// ```
    /// alphabet_1 alphabet_2 ...
    /// --------
    /// state_1 state_2 ...
    /// --------
    /// transition_matrix
    /// --------
    /// emission_matrix
    /// ```
    pub fn read_hmm(sections: &mut Iterator<Item = &str>) -> Result<Self, Error> {
        let alphabet = utils::parse_chars(
            sections
                .next()
                .ok_or_else(|| HMMError::InputFormatError("Missing alphabet".into()))?,
        );
        let states = utils::parse_chars(
            sections
                .next()
                .ok_or_else(|| HMMError::InputFormatError("Missing states".into()))?,
        );
        let alphabet_index = get_char_index(&alphabet);
        let state_index = get_char_index(&states);
        let transition_matrix = read_probability_matrix(
            sections
                .next()
                .ok_or_else(|| HMMError::InputFormatError("Missing transition matrix".into()))?,
            &state_index,
            &state_index,
        )?;
        let emission_matrix = read_probability_matrix(
            sections
                .next()
                .ok_or_else(|| HMMError::InputFormatError("Missing emission matrix".into()))?,
            &state_index,
            &alphabet_index,
        )?;
        Ok(HMM {
            alphabet,
            states,
            alphabet_index,
            state_index,
            transition_matrix,
            emission_matrix,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum State {
    Start,
    Match(usize),
    Insert(usize),
    Delete(usize),
    End,
}

impl State {
    fn to_emission_index(&self) -> Option<usize> {
        match self {
            State::Start => None,
            State::Match(_) => Some(0),
            State::Delete(_) => Some(1),
            State::Insert(_) => Some(2),
            State::End => None,
        }
    }

    fn to_transition_matrix_row_index(&self) -> Option<usize> {
        match self {
            State::Start => Some(1),
            State::End => None,
            State::Match(_) => Some(0),
            State::Delete(_) => Some(1),
            State::Insert(_) => Some(2),
        }
    }

    fn to_transition_matrix_col_index(&self) -> Option<usize> {
        match self {
            State::Start => None,
            State::End => Some(1),
            State::Match(_) => Some(1),
            State::Delete(_) => Some(2),
            State::Insert(_) => Some(0),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum SeedColumn {
    Active,
    Inactive,
}

#[derive(Fail, Debug)]
pub enum ProfileHMMError {
    #[fail(display = "Wrong input format: {}", _0)]
    InputFormatError(String),
    #[fail(display = "Invalid State: {:?}", _0)]
    InvalidStateError(State),
    #[fail(display = "Forbidden transition: {:?} -> {:?}", _0, _1)]
    ForbiddenTransitionError(State, State),
}


/// Row normalizes 3D matrix (across last axis)
fn normalize_matrix(matrix: &Array3<f32>) -> Array3<f32> {
    let mut matrix = matrix / &matrix.sum_axis(Axis(2)).insert_axis(Axis(2));
    let nan_indices: Vec<_> = matrix
        .indexed_iter()
        .filter_map(|(index, &item)| {
            if f32::is_nan(item) {
                Some(index)
            } else {
                None
            }
        })
        .collect();
    for index in nan_indices {
        matrix[index] = 0.;
    }
    matrix
}

fn format_line<T: ToString>(counts: impl Iterator<Item = T>) -> String {
    counts
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("\t")
}

#[derive(Debug)]
pub struct ProfileHMM {
    threshold: f32,
    alphabet: Vec<char>,
    alphabet_index: HashMap<char, usize>,
    gap_number: usize,
    msa: Vec<Vec<usize>>,
    seed_alignment: Vec<SeedColumn>,
    num_active: usize,
    // state, 3x3 transition matrix corresponding to M(i), D(i), I(i) -> I(i), M(i+1), D(i+1)
    // starts with S, M(0) -> I(0), M(1), D(1)
    // ends with M(n), D(n), I(n) -> I(n), E
    transition_matrix: Array3<f32>,
    // 0 = match, 1 = delete, 2 = insert
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
    ) -> Result<Self, Error> {
        let mut alphabet_index = alphabet_index;
        let mut alphabet = alphabet;
        alphabet.push('-');
        alphabet_index.insert('-', alphabet.len() - 1);
        let len_alphabet = alphabet.len();
        let gap_number = len_alphabet - 1;
        let msa: Vec<Vec<_>> = msa_section
            .split('\n')
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().chars().map(|c| alphabet_index[&c]).collect())
            .collect();
        let (num_sequences, length) = (msa.len(), msa[0].len());
        let mut num_active = 0;
        let seed_alignment = (0..length)
            .map(|i| {
                let num_gaps = msa
                    .iter()
                    .filter(|sequence| sequence[i] == gap_number)
                    .count();
                let gap_ratio = num_gaps as f32 / num_sequences as f32;
                if gap_ratio >= threshold {
                    SeedColumn::Inactive
                } else {
                    num_active += 1;
                    SeedColumn::Active
                }
            })
            .collect();
        let mut hmm = ProfileHMM {
            threshold,
            pseudocount: pseudocount.unwrap_or(0.),
            alphabet,
            alphabet_index,
            gap_number,
            msa,
            seed_alignment,
            num_active,
            transition_matrix: Array3::<f32>::zeros((num_active + 1, 3, 3)),
            emission_matrix: Array3::<f32>::zeros((3, num_active + 1, len_alphabet)),
        };
        hmm.calculate_matrices()?;
        Ok(hmm)
    }

    fn get_transition_matrix_index(
        &self,
        previous_state: State,
        next_state: State,
    ) -> Result<(usize, usize, usize), Error> {
        let index_0;
        match previous_state {
            State::Start => {
                index_0 = 0;
                let allowed = match next_state {
                    State::Start => false,
                    State::Match(i) | State::Delete(i) => i == 1,
                    State::Insert(i) => i == 0,
                    State::End => false,
                };
                if !allowed {
                    return Err(ProfileHMMError::ForbiddenTransitionError(
                        previous_state,
                        next_state,
                    )
                    .into());
                }
            }
            State::Match(i) | State::Delete(i) | State::Insert(i) => {
                index_0 = i;
                let allowed = match next_state {
                    State::Start => false,
                    State::Match(j) | State::Delete(j) => j == i + 1,
                    State::Insert(j) => j == i,
                    State::End => i == self.num_active,
                };
                if !allowed {
                    return Err(ProfileHMMError::ForbiddenTransitionError(
                        previous_state,
                        next_state,
                    )
                    .into());
                }
            }
            State::End => {
                return Err(
                    ProfileHMMError::ForbiddenTransitionError(previous_state, next_state).into(),
                )
            }
        }
        match (
            previous_state.to_transition_matrix_row_index(),
            next_state.to_transition_matrix_col_index(),
        ) {
            (Some(index_1), Some(index_2)) => Ok((index_0, index_1, index_2)),
            _ => Err(ProfileHMMError::ForbiddenTransitionError(previous_state, next_state).into()),
        }
    }

    fn calculate_matrices(&mut self) -> Result<(), Error> {
        let (mut previous_state, mut state);
        let mut active_j;
        for sequence in &self.msa {
            previous_state = State::Start;
            active_j = 0;
            for (j, character) in sequence.iter().enumerate() {
                match self.seed_alignment[j] {
                    SeedColumn::Active => {
                        active_j += 1;
                        if *character != self.gap_number {
                            self.emission_matrix[[0, active_j, *character]] += 1.;
                            state = Some(State::Match(active_j));
                        } else {
                            self.emission_matrix[[1, active_j, *character]] += 1.;
                            state = Some(State::Delete(active_j));
                        }
                    }
                    SeedColumn::Inactive => {
                        if *character != self.gap_number {
                            self.emission_matrix[[2, active_j, *character]] += 1.;
                            state = Some(State::Insert(active_j));
                        } else {
                            state = None;
                        }
                    }
                }
                if let Some(state) = state {
                    let index = self.get_transition_matrix_index(previous_state, state)?;
                    self.transition_matrix[index] += 1.;
                    previous_state = state;
                }
            }
            let index = self.get_transition_matrix_index(previous_state, State::End)?;
            self.transition_matrix[index] += 1.;
        }
        self.emission_matrix = normalize_matrix(&self.emission_matrix);
        self.transition_matrix = normalize_matrix(&self.transition_matrix);

        if self.pseudocount > 0. {
            self.emission_matrix += self.pseudocount;
            self.emission_matrix = normalize_matrix(&mut self.emission_matrix);
            self.transition_matrix += self.pseudocount;
            self.transition_matrix = normalize_matrix(&mut self.transition_matrix);
        }
        Ok(())
    }

    fn format_transition_line(&self, first: State) -> String {
        let num_states = 1 + 1 + self.num_active * 3 + 1;
        let mut probabilities = Vec::with_capacity(num_states);
        probabilities.push(0.);
        match first {
            State::Start => {
                probabilities.extend(self.transition_matrix.slice(s![0, 1, ..]).into_iter())
            }
            State::End => (),
            State::Match(i) | State::Delete(i) | State::Insert(i) => {
                let index = first.to_emission_index().unwrap();
                probabilities.extend((0..i).flat_map(|_| (0..3).map(|_| 0.)));
                let s = if i == self.num_active { 2 } else { 3 };
                probabilities.extend(self.transition_matrix.slice(s![i, index, ..s]).into_iter());
            }
        }
        if probabilities.len() < num_states {
            probabilities.extend((0..num_states - probabilities.len()).map(|_| 0.));
        }
        format_line(probabilities)
    }

    pub fn print_transition_matrix(&self) {
        for i in 0..=self.num_active {
            if i == 0 {
                print!("S\tI0\t")
            } else {
                print!("M{}\tD{}\tI{}\t", i, i, i);
            }
        }
        println!("E");
        for i in 0..=self.num_active {
            if i == 0 {
                println!("S\t{}", self.format_transition_line(State::Start));
                println!(
                    "I0\t{}",
                    self.format_transition_line(State::Insert(0))
                );
            } else {
                println!(
                    "M{}\t{}",
                    i,
                    self.format_transition_line(State::Match(i))
                );
                println!(
                    "D{}\t{}",
                    i,
                    self.format_transition_line(State::Delete(i))
                );
                println!(
                    "I{}\t{}",
                    i,
                    self.format_transition_line(State::Insert(i))
                );
            }
        }
        println!("E\t{}", self.format_transition_line(State::End));
    }

    pub fn print_emission_matrix(&self) {
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
