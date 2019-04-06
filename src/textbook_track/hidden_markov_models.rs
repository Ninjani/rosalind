use crate::utils;
use failure::Error;
use hashbrown::HashMap;
use ndarray::Array2;

#[derive(Fail, Debug)]
pub enum HMMError {
    #[fail(display = "Wrong input format: {}", _0)]
    FormatError(String),
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

impl HMM {
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
                .ok_or(HMMError::FormatError("Missing column names".into()))?,
        );
        let mut matrix = Array2::<f64>::zeros((row_indices.len(), column_indices.len()));
        for row in matrix_section {
            let mut parts = row.trim().split_whitespace();
            let state = parts
                .next()
                .ok_or(HMMError::FormatError("Missing row name".into()))?
                .chars()
                .next()
                .ok_or(HMMError::FormatError("State must be a character".into()))?;
            for (i, probability) in parts.enumerate() {
                matrix[[row_indices[&state], column_indices[&columns[i]]]] =
                    probability.parse::<f64>()?;
            }
        }
        Ok(matrix)
    }

    pub fn read_chars(line: &str) -> (Vec<char>, HashMap<char, usize>) {
        let chars = utils::parse_chars(line);
        let char_index = HMM::get_char_index(&chars);
        (chars, char_index)
    }

    /// list of chars to hashmap of char to index
    fn get_char_index(chars: &[char]) -> HashMap<char, usize> {
        chars.iter().enumerate().map(|(i, c)| (*c, i)).collect()
    }

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
                .ok_or(HMMError::FormatError("Missing alphabet".into()))?,
        );
        let states = utils::parse_chars(
            sections
                .next()
                .ok_or(HMMError::FormatError("Missing states".into()))?,
        );
        let alphabet_index = HMM::get_char_index(&alphabet);
        let state_index = HMM::get_char_index(&states);
        let transition_matrix = HMM::read_probability_matrix(
            sections
                .next()
                .ok_or(HMMError::FormatError("Missing transition matrix".into()))?,
            &state_index,
            &state_index,
        )?;
        let emission_matrix = HMM::read_probability_matrix(
            sections
                .next()
                .ok_or(HMMError::FormatError("Missing emission matrix".into()))?,
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
