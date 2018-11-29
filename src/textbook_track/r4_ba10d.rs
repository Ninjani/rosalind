use crate::textbook_track::hidden_markov_models::HMM;
use crate::utils;
use ndarray::Array2;

impl HMM {
    fn get_probability_of_sequence(&self, sequence: &str) -> f64 {
        let mut f_sums = Array2::<f64>::zeros((self.states.len(), sequence.len()));
        let mut sequence_chars = sequence.chars();
        let first_char = sequence_chars.next().unwrap();
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
        (0..self.states.len())
            .map(|k| f_sums[[k, sequence.len() - 1]])
            .sum()
    }
}

/// Compute the Probability of a String Emitted by an HMM
///
/// Given: A string x, followed by the alphabet Σ from which x was constructed, followed by the states States, transition matrix Transition, and emission matrix Emission of an HMM (Σ, States, Transition, Emission).
///
/// Return: The probability Pr(x) that the HMM emits x.
pub fn rosalind_ba10d() {
    let (hmm, sequence) = HMM::read_hmm(&utils::input_from_file(
        "data/textbook_track/rosalind_ba10d.txt",
    ));
    println!("{:e}", hmm.get_probability_of_sequence(&sequence));
}
