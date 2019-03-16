use crate::textbook_track::hidden_markov_models::HMM;
use crate::utils;
use hashbrown::HashMap;
use ndarray::Array2;

/// Compute the Probability of a Hidden Path
///
/// Given: A hidden path π followed by the states States and transition matrix Transition of an HMM (Σ, States, Transition, Emission).
///
/// Return: The probability of this path, Pr(π). You may assume that initial probabilities are equal.
pub fn rosalind_ba10a() {
    let (hmm, hidden_path) = HMM::read_hmm_transition(&utils::input_from_file(
        "data/textbook_track/rosalind_ba10a.txt",
    ));
    println!("{:e}", hmm.get_probability_of_hidden_path(&hidden_path));
}

impl HMM {
    /// Read HMM with format:
    ///
    /// ```
    /// hidden_path
    /// --------
    /// state_1 state_2 ...
    /// --------
    /// transition_matrix
    /// ```
    fn read_hmm_transition(contents: &str) -> (HMM, String) {
        let mut sections = contents.split("--------");
        let hidden_path = sections.next().unwrap().trim().to_owned();
        let states = utils::parse_chars(sections.next().unwrap());
        let state_index: HashMap<_, _> = states.iter().enumerate().map(|(i, c)| (*c, i)).collect();
        let emission_matrix = Array2::<f64>::zeros((states.len(), 1));
        let transition_matrix =
            HMM::read_probability_matrix(sections.next().unwrap(), &state_index, &state_index);
        (
            HMM {
                alphabet: Vec::new(),
                states,
                alphabet_index: HashMap::new(),
                state_index,
                transition_matrix,
                emission_matrix,
            },
            hidden_path,
        )
    }

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
