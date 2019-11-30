use failure::Error;
use ndarray::Array2;

use hidden_markov_models::{HMM, HMMError};
use utility;
use utility::math::Comparable;

/// Implement the Viterbi Algorithm
///
/// Given: A string x, followed by the alphabet Σ from which x was constructed,
/// followed by the states States, transition matrix Transition,
/// and emission matrix Emission of an HMM (Σ, States, Transition, Emission).
///
/// Return: A path that maximizes the (unconditional) probability Pr(x, π) over all possible paths π.
pub fn rosalind_ba10c(filename: &str) -> Result<String, Error> {
    let contents = utility::io::input_from_file(filename)?;
    let mut sections = contents.split("--------");
    let sequence = sections
        .next()
        .ok_or_else(|| HMMError::InputFormatError("Missing sequence".into()))?
        .trim()
        .to_owned();
    let hmm = HMM::read_hmm(&mut sections)?;
    let path = hmm.run_viterbi(&sequence)?;
    println!("{}", path);
    Ok(path)
}

pub trait Viterbi {
    fn run_viterbi(&self, sequence: &str) -> Result<String, Error>;
}

impl Viterbi for HMM {
    /// Runs the viterbi algorithm on a sequence
    fn run_viterbi(&self, sequence: &str) -> Result<String, Error> {
        let mut v_maxes = Array2::<f64>::zeros((self.states.len(), sequence.len()));
        let mut pointers = Array2::<usize>::zeros((self.states.len(), sequence.len()));
        let mut sequence_chars = sequence.chars();
        let first_char = sequence_chars
            .next()
            .ok_or_else(|| HMMError::InputFormatError("Empty sequence".into()))?;
        for k in 0..self.states.len() {
            v_maxes[[k, 0]] = (self.emission_matrix[[k, self.alphabet_index[&first_char]]] * 1.)
                / self.states.len() as f64;
        }
        for (i, s_char) in sequence_chars.enumerate() {
            for k in 0..self.states.len() {
                let (max_index, max_value) = f64::argmax_max(
                    &(0..self.states.len())
                        .map(|j| self.transition_matrix[[j, k]] * v_maxes[[j, i]])
                        .collect::<Vec<_>>(),
                );
                v_maxes[[k, i + 1]] =
                    max_value * self.emission_matrix[[k, self.alphabet_index[&s_char]]];
                pointers[[k, i + 1]] = max_index;
            }
        }
        let (mut max_index, _) = f64::argmax_max(
            &((0..self.states.len())
                .map(|k| v_maxes[[k, sequence.len() - 1]])
                .collect::<Vec<_>>()),
        );
        let mut path = vec![self.states[max_index]];
        for i in (1..sequence.len()).rev() {
            max_index = pointers[[max_index, i]];
            path.push(self.states[max_index]);
        }
        Ok(path.into_iter().rev().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ba10c() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_ba10c")?;
        let output = utility::io::input_from_file(&output_file)?;
        assert_eq!(rosalind_ba10c(&input_file)?, output);
        Ok(())
    }
}
