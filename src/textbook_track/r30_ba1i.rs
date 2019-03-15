use crate::stronghold::r6_hamm::hamming;
use crate::textbook_track::r23_ba1b::get_most_frequent_kmers;
use crate::utils;
use crate::utils::Parseable;
use hashbrown::{HashMap, HashSet};
use failure::Error;

pub fn rosalind_ba1i() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba1i.txt");
    let lines = contents.split('\n').collect::<Vec<_>>();
    let numbers = usize::parse_line(lines[1])?;
    let (k, mismatch) = (numbers[0], numbers[1]);
    let counts_tuple = get_sorted_kmer_counts_approx(lines[0], k, mismatch);
    utils::print_array(&get_most_frequent_kmers(&counts_tuple));
    Ok(())
}

pub fn get_mismatch_sequences(sequence: &str, mismatch: usize) -> Vec<String> {
    if mismatch == 0 {
        vec![sequence.to_owned()]
    } else if sequence.len() == 1 {
        "ACGT".chars().map(|c| c.to_string()).collect()
    } else {
        let mut mismatch_sequences = HashSet::new();
        let sequence: Vec<_> = sequence.chars().collect();
        let suffix: String = sequence[1..].iter().collect();
        let suffix_mismatch_sequences = get_mismatch_sequences(&suffix, mismatch);
        for neighbor in suffix_mismatch_sequences {
            if hamming(&suffix, &neighbor) < mismatch {
                for c in "ACGT".chars() {
                    mismatch_sequences.insert(format!("{}{}", c, neighbor));
                }
            } else {
                mismatch_sequences.insert(format!("{}{}", sequence[0], neighbor));
            }
        }
        mismatch_sequences.into_iter().collect()
    }
}

fn get_sorted_kmer_counts_approx(text: &str, k: usize, mismatch: usize) -> Vec<(String, usize)> {
    let text: Vec<_> = text.chars().collect();
    let mut counts = HashMap::new();
    for i in 0..=(text.len() - k) {
        let text_kmer: String = text[i..(i + k)].iter().collect();
        for kmer in get_mismatch_sequences(&text_kmer, mismatch) {
            *counts.entry(kmer).or_insert(0usize) += 1;
        }
    }
    let mut counts_tuple = counts.into_iter().collect::<Vec<_>>();
    counts_tuple.sort_by(|a, b| b.1.cmp(&a.1));
    counts_tuple
}
