use crate::textbook_track::hidden_markov_models::{HMMError, HMM, read_chars, read_probability_matrix};
use crate::utils;
use failure::Error;
use hashbrown::HashMap;
use ndarray::{Array1, Array2};

/// Compute the Probability of an Outcome Given a Hidden Path
///
/// Given: A string x, followed by the alphabet Σ from which x was constructed,
/// followed by a hidden path π, followed by the states States and emission matrix Emission
/// of an HMM (Σ, States, Transition, Emission).
///
/// Return: The conditional probability Pr(x|π) that string x will be emitted by the HMM given the hidden path π.
pub fn rosalind_ba10b() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba10b.txt");
    let mut sections = contents.split("--------");
    let sequence = sections
        .next()
        .ok_or_else(|| HMMError::InputFormatError("Missing sequence".into()))?
        .trim()
        .to_owned();
    let (alphabet, alphabet_index) = read_chars(
        &sections
            .next()
            .ok_or_else(|| HMMError::InputFormatError("Missing alphabet".into()))?,
    );
    let hidden_path = sections
        .next()
        .ok_or_else(|| HMMError::InputFormatError("Missing hidden path".into()))?
        .trim()
        .to_owned();
    let (states, state_index) = read_chars(
        &sections
            .next()
            .ok_or_else(|| HMMError::InputFormatError("Missing states".into()))?,
    );
    let transition_matrix = HMM::transition_matrix_from_path(&hidden_path, &state_index);
    let emission_matrix = read_probability_matrix(
        sections
            .next()
            .ok_or_else(|| HMMError::InputFormatError("Missing emission matrix".into()))?,
        &state_index,
        &alphabet_index,
    )?;
    let hmm = HMM {
        alphabet,
        states,
        alphabet_index,
        state_index,
        transition_matrix,
        emission_matrix,
    };
    println!(
        "{:e}",
        hmm.get_probability_of_sequence_given_path(&sequence, &hidden_path)
    );
    Ok(())
}

impl HMM {
    fn transition_matrix_from_path(
        hidden_path: &str,
        state_index: &HashMap<char, usize>,
    ) -> Array2<f64> {
        let mut transition_matrix = Array2::<f64>::zeros((state_index.len(), state_index.len()));
        let mut start_counts = Array1::<f64>::zeros(state_index.len());
        for (path_char, next_path_char) in hidden_path.chars().zip(hidden_path.chars().skip(1)) {
            transition_matrix[[state_index[&path_char], state_index[&next_path_char]]] += 1.;
            start_counts[state_index[&path_char]] += 1.;
        }
        for (i, count) in start_counts.iter().enumerate() {
            let mut row = transition_matrix.slice_mut(s![i, ..]);
            row /= *count;
        }
        transition_matrix
    }

    fn get_probability_of_sequence_given_path(&self, sequence: &str, hidden_path: &str) -> f64 {
        let mut probability = 1.;
        for (path_char, sequence_char) in hidden_path.chars().zip(sequence.chars()) {
            probability *= self.emission_matrix[[
                self.state_index[&path_char],
                self.alphabet_index[&sequence_char],
            ]];
        }
        probability
    }
}
