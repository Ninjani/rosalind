use std::collections::HashMap;

use failure::Error;
use ndarray::{Array2, Array3, Axis};

use utility;
use utility::io::Parseable;

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

/// Read a list of space-separated characters and return them
/// along with a HashMap mapping each character to its index
pub fn get_chars_and_index(line: &str) -> Result<(Vec<char>, HashMap<char, usize>), Error> {
    let chars = char::parse_line(line)?;
    let char_index = get_char_index(&chars);
    Ok((chars, char_index))
}

/// list of chars to HashMap of char to index
fn get_char_index(chars: &[char]) -> HashMap<char, usize> {
    chars.iter().enumerate().map(|(i, c)| (*c, i)).collect()
}

/// Read a matrix
/// e.g.
///     A   B
/// A   0.194   0.806
/// B   0.273   0.727
///
/// row_indices maps row names to indices in returned matrix
/// col_indices maps col names to indices in returned matrix
pub fn read_probability_matrix(
    contents: &str,
    row_indices: &HashMap<char, usize>,
    column_indices: &HashMap<char, usize>,
) -> Result<Array2<f64>, Error> {
    let mut matrix_section = contents.trim().split('\n');
    let columns = char::parse_line(
        matrix_section
            .next()
            .ok_or_else(|| HMMError::InputFormatError("Missing column names".into()))?,
    )?;
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
    pub fn read_hmm(sections: &mut dyn Iterator<Item=&str>) -> Result<Self, Error> {
        let alphabet = char::parse_line(
            sections
                .next()
                .ok_or_else(|| HMMError::InputFormatError("Missing alphabet".into()))?,
        )?;
        let states = char::parse_line(
            sections
                .next()
                .ok_or_else(|| HMMError::InputFormatError("Missing states".into()))?,
        )?;
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
/// Replaces NaNs with zeros
fn normalize_matrix(matrix: &Array3<f32>) -> Array3<f32> {
    let mut matrix = matrix / &matrix.sum_axis(Axis(2)).insert_axis(Axis(2));
    let nan_indices: Vec<_> = matrix
        .indexed_iter()
        .filter_map(
            |(index, &item)| {
                if f32::is_nan(item) {
                    Some(index)
                } else {
                    None
                }
            },
        )
        .collect();
    for index in nan_indices {
        matrix[index] = 0.;
    }
    matrix
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
    /// Transition matrix is 3D with shape (num_active + 1, 3, 3)
    /// One 3x3 sub-matrix for each set of active states
    /// 3x3 matrix is of the form:
    ///     I(i) M(i+1) D(i+1)
    /// M(i)
    /// D(i)
    /// I(i)
    ///
    /// First sub-matrix (matrix[0, .., ..])
    ///     I(0) M(1) D(1)
    ///     0     0    0
    /// S
    /// I(0)
    ///
    /// Last sub-matrix (matrix[num_active, .., ..])
    ///     I(n) E
    /// M(n)       0
    /// D(n)       0
    /// I(n)       0
    transition_matrix: Array3<f32>,
    /// Emission matrix is 3D with shape (3, num_active + 1, len_alphabet)
    /// 0 = match emissions
    /// 1 = delete emissions
    /// 2 = insert emissions
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
        let index_0 = match previous_state {
            State::Start => {
                let allowed = match next_state {
                    State::Start => false,
                    State::Match(i) | State::Delete(i) => i == 1,
                    State::Insert(i) => i == 0,
                    State::End => false,
                };
                if allowed {
                    Some(0)
                } else {
                    None
                }
            }
            State::Match(i) | State::Delete(i) | State::Insert(i) => {
                let allowed = match next_state {
                    State::Start => false,
                    State::Match(j) | State::Delete(j) => j == i + 1,
                    State::Insert(j) => j == i,
                    State::End => i == self.num_active,
                };
                if allowed {
                    Some(i)
                } else {
                    None
                }
            }
            State::End => None,
        };
        match (
            index_0,
            previous_state.to_transition_matrix_row_index(),
            next_state.to_transition_matrix_col_index(),
        ) {
            (Some(index_0), Some(index_1), Some(index_2)) => Ok((index_0, index_1, index_2)),
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
            self.emission_matrix = normalize_matrix(&self.emission_matrix);
            self.transition_matrix += self.pseudocount;
            self.transition_matrix = normalize_matrix(&self.transition_matrix);
        }
        Ok(())
    }

    fn get_transition_line(&self, from_state: State) -> Vec<f32> {
        let num_states = 1 + 1 + self.num_active * 3 + 1;
        let mut probabilities = Vec::with_capacity(num_states);
        probabilities.push(0.);
        match from_state {
            State::Start => {
                probabilities.extend(self.transition_matrix.slice(s![0, 1, ..]).into_iter())
            }
            State::End => (),
            State::Match(i) | State::Delete(i) | State::Insert(i) => {
                let index = from_state.to_emission_index().unwrap();
                probabilities.extend((0..i).flat_map(|_| (0..3).map(|_| 0.)));
                let s = if i == self.num_active { 2 } else { 3 };
                probabilities.extend(self.transition_matrix.slice(s![i, index, ..s]).into_iter());
            }
        }
        if probabilities.len() < num_states {
            probabilities.extend((0..num_states - probabilities.len()).map(|_| 0.));
        }
        probabilities
    }

    pub fn print_transition_matrix(&self) {
        // Header
        print!("S\tI0\t");
        for i in 1..=self.num_active {
            print!("M{}\tD{}\tI{}\t", i, i, i);
        }
        println!("E");

        // Matrix
        println!(
            "S\t{}",
            utility::io::format_line(self.get_transition_line(State::Start).into_iter(), "\t")
        );
        println!(
            "I0\t{}",
            utility::io::format_line(self.get_transition_line(State::Insert(0)).into_iter(), "\t")
        );
        for i in 1..=self.num_active {
            for (state_name, state) in "MDI"
                .chars()
                .zip(vec![State::Match(i), State::Delete(i), State::Insert(i)].into_iter())
                {
                    println!(
                        "{}{}\t{}",
                        state_name,
                        i,
                        utility::io::format_line(self.get_transition_line(state).into_iter(), "\t")
                    );
                }
        }
        println!(
            "E\t{}",
            utility::io::format_line(self.get_transition_line(State::End).into_iter(), "\t")
        );
    }

    pub fn print_emission_matrix(&self) {
        let len_alphabet_nogap = self.alphabet.len() - 1;
        // Header
        println!(
            "\t{}",
            utility::io::format_line(self.alphabet.iter().take(len_alphabet_nogap), "\t")
        );

        // Matrix
        println!(
            "S\t{}",
            utility::io::format_line((0..len_alphabet_nogap).map(|_| 0), "\t")
        );
        println!(
            "I0\t{}",
            utility::io::format_line(
                self.emission_matrix
                    .slice(s![0, 0, ..len_alphabet_nogap])
                    .iter(),
                "\t",
            )
        );
        for i in 1..=self.num_active {
            for (state_name, index) in "MDI".chars().zip(0..3) {
                println!(
                    "{}{}\t{}",
                    state_name,
                    i,
                    utility::io::format_line(
                        self.emission_matrix
                            .slice(s![index, i, ..len_alphabet_nogap])
                            .iter(),
                        "\t",
                    )
                );
            }
        }
        println!(
            "E\t{}",
            utility::io::format_line((0..len_alphabet_nogap).map(|_| 0), "\t")
        );
    }
}
