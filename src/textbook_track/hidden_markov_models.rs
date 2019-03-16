use crate::utils;
use hashbrown::HashMap;
use ndarray::Array2;

/// Hidden Markov Model (Implementations spread across files)
pub struct HMM {
    pub alphabet: Vec<char>,
    pub states: Vec<char>,
    pub alphabet_index: HashMap<char, usize>,
    pub state_index: HashMap<char, usize>,
    pub transition_matrix: Array2<f64>,
    pub emission_matrix: Array2<f64>,
}

impl HMM {
    /// Read a matrix
    pub fn read_probability_matrix(
        contents: &str,
        row_indices: &HashMap<char, usize>,
        column_indices: &HashMap<char, usize>,
    ) -> Array2<f64> {
        let mut matrix_section = contents.trim().split('\n');
        let columns = utils::parse_chars(matrix_section.next().unwrap());
        let mut matrix = Array2::<f64>::zeros((row_indices.len(), column_indices.len()));
        for row in matrix_section {
            let mut parts = row.trim().split_whitespace();
            let state = parts.next().unwrap().chars().next().unwrap();
            for (i, probability) in parts.enumerate() {
                matrix[[row_indices[&state], column_indices[&columns[i]]]] =
                    probability.parse::<f64>().unwrap();
            }
        }
        matrix
    }

    /// Read HMM in format:
    /// ```
    /// sequence
    /// --------
    /// alphabet_1 alphabet_2 ...
    /// --------
    /// state_1 state_2 ...
    /// --------
    /// transition_matrix
    /// --------
    /// emission_matrix
    /// ```
    pub fn read_hmm(contents: &str) -> (HMM, String) {
        let mut sections = contents.split("--------");
        let sequence = sections.next().unwrap().trim().to_owned();
        let alphabet = utils::parse_chars(sections.next().unwrap());
        let states = utils::parse_chars(sections.next().unwrap());
        let alphabet_index: HashMap<_, _> =
            alphabet.iter().enumerate().map(|(i, c)| (*c, i)).collect();
        let state_index: HashMap<_, _> = states.iter().enumerate().map(|(i, c)| (*c, i)).collect();
        let transition_matrix =
            HMM::read_probability_matrix(sections.next().unwrap(), &state_index, &state_index);
        let emission_matrix =
            HMM::read_probability_matrix(sections.next().unwrap(), &state_index, &alphabet_index);
        (
            HMM {
                alphabet,
                states,
                alphabet_index,
                state_index,
                transition_matrix,
                emission_matrix,
            },
            sequence,
        )
    }
}
