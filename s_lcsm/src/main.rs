use bio::data_structures::suffix_array as SA;
use failure::Error;

use crate::utility;

const SENTINEL: &str = "$";

/// Finding a Shared Motif
///
/// Given: A collection of k (k≤100) DNA strings of length at most 1 kbp each in FASTA format.
///
/// Return: A longest common substring of the collection.
/// (If multiple solutions exist, you may return any single solution.)
pub fn rosalind_lcsm(filename: &str) -> Result<String, Error> {
    let sequences = utility::io::read_fasta_file(filename)?
        .values()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    let output = LongestCommonMotif::get_lcm(sequences);
    Ok(output)
}

/// Computing Longest Common Substrings Using Suffix Arrays (Maxim A. Babenko, Tatiana A. Starikovskaya, 2008)
pub struct LongestCommonMotif {
    sequences: Vec<String>,
    n_sequences: usize,
    combined_sequence: String,
    types: Vec<usize>,
    suffix_array: Vec<usize>,
    lcp_array: Vec<isize>,
    counts: Vec<usize>,
    n_pos: usize,
}

impl LongestCommonMotif {
    fn new(sequences: Vec<String>) -> LongestCommonMotif {
        let n_sequences = sequences.len();
        let combined_sequence = sequences.join(SENTINEL) + SENTINEL;
        let types = sequences
            .iter()
            .enumerate()
            .flat_map(|(i, s)| (0..=s.len()).map(move |_| i))
            .collect::<Vec<usize>>();
        let suffix_array = SA::suffix_array(combined_sequence.as_bytes());
        let lcp_array = SA::lcp(combined_sequence.as_bytes(), &suffix_array).decompress();
        LongestCommonMotif {
            sequences,
            n_sequences,
            combined_sequence,
            types,
            suffix_array,
            lcp_array,
            counts: (0..n_sequences).map(|_| 0).collect(),
            n_pos: 0,
        }
    }

    fn get_type(&self, index: usize) -> Option<usize> {
        match self.suffix_array.get(index) {
            Some(index) => self.types.get(*index).cloned(),
            None => None,
        }
    }

    fn increment(&mut self, sequence_type: usize) {
        self.counts[sequence_type] += 1;
        if self.counts[sequence_type] == 1 {
            self.n_pos += 1;
        }
    }

    fn decrement(&mut self, sequence_type: usize) {
        self.counts[sequence_type] -= 1;
        if self.counts[sequence_type] == 0 {
            self.n_pos -= 1;
        }
    }

    fn get_mins(&self, k_goods: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut k_mins = Vec::new();
        for k_segment in k_goods {
            let (delta_minus, delta_plus) = k_segment;
            let min_lcp = *self.lcp_array[(delta_minus + 1)..=delta_plus]
                .iter()
                .min()
                .unwrap() as usize;
            k_mins.push((self.suffix_array[delta_minus], min_lcp));
        }
        k_mins
    }

    fn get_motif(&self, k_mins: Vec<(usize, usize)>) -> String {
        let (index, length) = k_mins.into_iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
        let motif = &self.combined_sequence[index..(index + length)];
        if motif.ends_with(SENTINEL) {
            motif[..(length - 1)].to_owned()
        } else {
            motif.to_owned()
        }
    }

    fn make_k_good(&mut self, k: usize) -> Vec<(usize, usize)> {
        let mut delta_minus = self.n_sequences;
        let mut delta_plus = self.n_sequences - 1;
        let mut k_good_segments = Vec::new();
        for _ in self.n_sequences..self.suffix_array.len() {
            while self.n_pos < k {
                delta_plus += 1;
                match self.get_type(delta_plus) {
                    Some(sequence_type) => {
                        self.increment(sequence_type);
                    }
                    None => return k_good_segments,
                }
            }
            k_good_segments.push((delta_minus, delta_plus));
            match self.get_type(delta_minus) {
                Some(sequence_type) => {
                    self.decrement(sequence_type);
                }
                None => return k_good_segments,
            }
            delta_minus += 1;
        }
        k_good_segments
    }

    pub fn get_lcm(sequences: Vec<String>) -> String {
        let k = sequences.len();
        let mut lcm = LongestCommonMotif::new(sequences);
        let k_good_segments = lcm.make_k_good(k);
        let k_mins = lcm.get_mins(k_good_segments);
        lcm.get_motif(k_mins)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcsm() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_lcsm")?;
        let output = utility::io::input_from_file(&output_file)?
            .trim()
            .to_owned();
        let result = rosalind_lcsm(&input_file)?;
        let sequences = utility::io::read_fasta_file(&input_file)?
            .values()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        assert_eq!(output.len(), result.len());
        assert!(sequences
            .into_iter()
            .all(|sequence| sequence.contains(&result)));
        Ok(())
    }
}
