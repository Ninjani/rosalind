use std::collections::HashMap;

use bio::pattern_matching::{bom, shift_and};
use itertools::Itertools;

/// Count occurrences of each character in a string
pub fn char_counter(input: &str) -> HashMap<char, usize> {
    let mut counter = HashMap::new();
    for character in input.chars() {
        *counter.entry(character).or_insert(0) += 1;
    }
    counter
}

/// Exact string search for overlapping motifs in a string (No regex).
pub fn find_motifs(motif: &str, string: &str) -> Vec<usize> {
    if motif.len() < 64 {
        let matcher = shift_and::ShiftAnd::new(motif.as_bytes());
        matcher.find_all(string.as_bytes()).collect()
    } else {
        let matcher = bom::BOM::new(motif.as_bytes());
        matcher.find_all(string.as_bytes()).collect()
    }
}

/// Chunk a string into sub-strings
pub fn sub_strings(source: &str, sub_size: usize) -> Vec<String> {
    source
        .chars()
        .chunks(sub_size)
        .into_iter()
        .map(|c| c.collect())
        .collect()
}

/// Return overlapping kmers of a given length from a string
pub fn kmerize(string: &str, length: usize) -> Vec<String> {
    string
        .chars()
        .collect::<Vec<_>>()
        .windows(length)
        .map(|chunk| chunk.iter().cloned().collect::<String>())
        .collect()
}

/// Get hamming distance between two equal-length strings
pub fn hamming(string_1: &str, string_2: &str) -> usize {
    string_1
        .chars()
        .zip(string_2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}
