use hashbrown::HashMap;
use itertools::Itertools;
use ndarray::Array2;
use std::iter::repeat;
use utils;

// Work In Progress!

#[derive(Clone, Copy, Debug)]
enum State {
    Start,
    Match,
    Insert,
    Delete,
    End,
}

struct ProfileHMM {
    alphabet: Vec<char>,
    alphabet_index: HashMap<char, usize>,
    msa: Vec<Vec<usize>>,
    states: Vec<State>,
    state_pairs: Vec<(State, State)>,
    match_emissions: Array2<f64>,
    insert_emissions: Array2<f64>,
    state_transitions: HashMap<(State, State), f64>,
}

impl ProfileHMM {
    fn initialize(contents: &str) -> Self {
        let mut sections = contents.split("--------");
        let threshold = sections.next().unwrap().trim().parse::<f64>().unwrap();
        let alphabet = utils::parse_chars(sections.next().unwrap());
        let len_alphabet = alphabet.len();
        let mut alphabet_index: HashMap<_, _> =
            alphabet.iter().enumerate().map(|(i, c)| (*c, i)).collect();
        alphabet_index.insert('-', len_alphabet);
        let msa: Vec<Vec<_>> = sections
            .next()
            .unwrap()
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| alphabet_index[&c])
                    .collect::<Vec<_>>()
            })
            .collect();
        let len_sequence = msa[0].len();
        let match_states = Self::find_match_states(&msa, len_alphabet + 1, len_sequence, threshold);
        let num_match_states = match_states.len();
        let state_transitions = HashMap::new();
        let num_state_pairs = state_pairs.len();
        ProfileHMM {
            alphabet,
            alphabet_index,
            msa,
            states,
            state_pairs,
            match_emissions: Array2::<f64>::zeros((len_alphabet, num_match_states + 1)),
            insert_emissions: Array2::<f64>::zeros((len_alphabet, num_match_states + 1)),
            state_transitions,
        }
    }

    fn get_matrices(&mut self) {
        let mut previous_state = State::Start;
        let mut state;
        for i in 0..self.msa[0].len() {
            let mut counts: Vec<_> = repeat(0).take(num_alphabets).collect();
            for sequence in sequences {
                counts[sequence[i]] += 1;
            }
            if f64::from(counts[num_alphabets - 1]) / length as f64 > threshold {
                state = State::Match;
            } else {
                state = State::Insert;
            }
            for sequence in sequences {
                if sequence[i] == num_alphabets {
                    match state {
                       // State::Match => self.state_transitions.entry((previous_state))
                    }
                }
            }
        }
    }
    fn count_match_emissions(&mut self) {
        {
            let mut start_state = self.match_emissions.slice_mut(s![.., 0]);
            start_state.fill(-1.);
        }
        let num_match_states = self.match_states.len();
        for i in 1..=num_match_states {
            for sequence in &self.msa {
                let character = sequence[self.match_states[i - 1]];
                if character < self.alphabet.len() {
                    self.match_emissions[[character, i]] += 1.;
                }
            }
        }
    }

    fn find_match_states(
        sequences: &[Vec<usize>],
        num_alphabets: usize,
        length: usize,
        threshold: f64,
    ) -> Vec<State> {
        let mut match_states = Vec::new();
        for i in 0..length {
            let mut counts: Vec<_> = repeat(0).take(num_alphabets).collect();
            for sequence in sequences {
                counts[sequence[i]] += 1;
            }
            if f64::from(counts[num_alphabets - 1]) / length as f64 > threshold {
                match_states.push(State::Insert);
            } else {
                match_states.push(State::Match);
            }
        }
        match_states
    }
}

pub fn rosalind_ba10e() {
    let mut profile_hmm = ProfileHMM::initialize(&utils::input_from_file(
        "data/textbook_track/rosalind_ba10e.txt",
    ));
    profile_hmm.count_match_emissions();
    println!("{:?}", profile_hmm.match_emissions);
}
