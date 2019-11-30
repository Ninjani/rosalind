use std::collections::HashMap;

use failure::Error;
use ndarray::Array2;

use hidden_markov_models::{
    get_chars_and_index, HMM, HMMError, read_probability_matrix,
};
use utility;

/// Compute the Probability of a Hidden Path
///
/// Given: A hidden path π followed by the states States and transition matrix Transition of an HMM (Σ, States, Transition, Emission).
///
/// Return: The probability of this path, Pr(π). You may assume that initial probabilities are equal.
pub fn rosalind_ba10a(filename: &str) -> Result<f64, Error> {
    let contents = utility::io::input_from_file(filename)?;
    let mut sections = contents.split("--------");
    let hidden_path = sections
        .next()
        .ok_or_else(|| HMMError::InputFormatError("Missing hidden path".into()))?
        .trim()
        .to_owned();
    let (states, state_index) = get_chars_and_index(
        &sections
            .next()
            .ok_or_else(|| HMMError::InputFormatError("Missing states".into()))?,
    )?;
    let transition_matrix = read_probability_matrix(
        sections
            .next()
            .ok_or_else(|| HMMError::InputFormatError("Missing transition matrix".into()))?,
        &state_index,
        &state_index,
    )?;
    let emission_matrix = Array2::<f64>::zeros((states.len(), 1));
    let hmm = HMM {
        alphabet: Vec::new(),
        states,
        alphabet_index: HashMap::new(),
        state_index,
        transition_matrix,
        emission_matrix,
    };
    let probability = hmm.get_probability_of_hidden_path(&hidden_path);
    println!("{:e}", probability);
    Ok(probability)
}

pub trait HiddenPathProbability {
    fn get_probability_of_hidden_path(&self, hidden_path: &str) -> f64;
}

impl HiddenPathProbability for HMM {
    fn get_probability_of_hidden_path(&self, hidden_path: &str) -> f64 {
        let mut probability = 1. / (self.states.len() as f64);
        for (current_char, next_char) in hidden_path.chars().zip(hidden_path.chars().skip(1)) {
            probability *= self.transition_matrix[[
                self.state_index[&current_char],
                self.state_index[&next_char],
            ]];
        }
        probability
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn ba10a() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_ba10a")?;
        let output = utility::io::input_from_file(&output_file)?.parse::<f64>()?;
        assert_approx_eq!(
            rosalind_ba10a(&input_file)?,
            output,
            utility::testing::ROSALIND_FLOAT_ERROR_F64
        );
        Ok(())
    }
}
