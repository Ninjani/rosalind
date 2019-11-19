use failure::Error;
use ndarray::Array2;

use crate::utility;
use crate::utility::hidden_markov_models::{HMM, HMMError};

/// Compute the Probability of a String Emitted by an HMM
///
/// Given: A string x, followed by the alphabet Σ from which x was constructed,
/// followed by the states States, transition matrix Transition,
/// and emission matrix Emission of an HMM (Σ, States, Transition, Emission).
///
/// Return: The probability Pr(x) that the HMM emits x.
pub fn rosalind_ba10d(filename: &str) -> Result<f64, Error> {
    let contents = utility::io::input_from_file(filename)?;
    let mut sections = contents.split("--------");
    let sequence = sections
        .next()
        .ok_or_else(|| HMMError::InputFormatError("Missing sequence".into()))?
        .trim()
        .to_owned();
    let hmm = HMM::read_hmm(&mut sections)?;
    let probability = hmm.get_probability_of_sequence(&sequence)?;
    println!("{:e}", probability);
    Ok(probability)
}

impl HMM {
    fn get_probability_of_sequence(&self, sequence: &str) -> Result<f64, Error> {
        let mut f_sums = Array2::<f64>::zeros((self.states.len(), sequence.len()));
        let mut sequence_chars = sequence.chars();
        let first_char = sequence_chars
            .next()
            .ok_or_else(|| HMMError::InputFormatError("Empty sequence".into()))?;
        for k in 0..self.states.len() {
            f_sums[[k, 0]] = self.emission_matrix[[k, self.alphabet_index[&first_char]]] * 1.
                / self.states.len() as f64;
        }
        for (i, s_char) in sequence_chars.enumerate() {
            for k in 0..self.states.len() {
                f_sums[[k, i + 1]] = (0..self.states.len())
                    .map(|j| self.transition_matrix[[j, k]] * f_sums[[j, i]])
                    .sum::<f64>()
                    * self.emission_matrix[[k, self.alphabet_index[&s_char]]];
            }
        }
        Ok((0..self.states.len())
            .map(|k| f_sums[[k, sequence.len() - 1]])
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn ba10d() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_ba10d")?;
        let output: f64 = utility::io::input_from_file(&output_file)?.parse()?;
        assert_approx_eq!(
            rosalind_ba10d(&input_file)?,
            output,
            utility::testing::ROSALIND_FLOAT_ERROR_F64
        );
        Ok(())
    }
}
