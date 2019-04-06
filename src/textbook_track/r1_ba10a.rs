use crate::textbook_track::hidden_markov_models::{HMMError, HMM};
use crate::utils;
use failure::Error;
use hashbrown::HashMap;
use ndarray::Array2;

/// Compute the Probability of a Hidden Path
///
/// Given: A hidden path π followed by the states States and transition matrix Transition of an HMM (Σ, States, Transition, Emission).
///
/// Return: The probability of this path, Pr(π). You may assume that initial probabilities are equal.
pub fn rosalind_ba10a() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba10a.txt");
    let mut sections = contents.split("--------");
    let hidden_path = sections
        .next()
        .ok_or(HMMError::FormatError("Missing hidden path".into()))?
        .trim()
        .to_owned();
    let (states, state_index) = HMM::read_chars(
        &sections
            .next()
            .ok_or(HMMError::FormatError("Missing states".into()))?,
    );
    let transition_matrix = HMM::read_probability_matrix(
        sections
            .next()
            .ok_or(HMMError::FormatError("Missing transition matrix".into()))?,
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
    println!("{:e}", hmm.get_probability_of_hidden_path(&hidden_path));
    Ok(())
}

impl HMM {
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
