use crate::utils;
use std::collections::HashMap;

/// Get frequencies of each nucleotide at each position in a collection of sequences (profile)
fn get_profile(sequences: &[&str]) -> Vec<HashMap<char, usize>> {
    let sequence_length = sequences[0].len();
    let mut profile = Vec::with_capacity(sequences.len());
    for i in 0..sequence_length {
        let position_string = sequences
            .iter()
            .map(|sequence| sequence.chars().nth(i).unwrap())
            .collect::<String>();
        profile.push(utils::char_counter(&position_string));
    }
    profile
}

/// Get consensus sequence from a profile
fn get_consensus(profile: &[HashMap<char, usize>]) -> String {
    let mut consensus = String::with_capacity(profile.len());
    for counts in profile.iter() {
        let mut count_vec: Vec<_> = counts.iter().collect();
        count_vec.sort_by(|a, b| a.1.cmp(b.1).reverse());
        consensus.push(*count_vec[0].0);
    }
    consensus
}

/// Profile pretty-printer
fn print_profile(profile: &[HashMap<char, usize>]) {
    for nucleotide in "ACGT".chars() {
        println!(
            "{}: {}",
            nucleotide,
            profile
                .iter()
                .map(|counts| counts.get(&nucleotide).unwrap_or(&0).to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}

/// Consensus and Profile
///
/// Given: A collection of at most 10 DNA strings of equal length (at most 1 kbp) in FASTA format.
///
/// Return: A consensus string and profile matrix for the collection. (If several possible consensus strings exist, then you may return any one of them.)
pub fn rosalind_cons() {
    let contents = utils::read_fasta_file("data/stronghold/rosalind_cons.txt");
    let sequences = contents.values().map(|s| s.as_ref()).collect::<Vec<&str>>();
    let profile = get_profile(&sequences);
    let consensus = get_consensus(&profile);
    println!("{}", consensus);
    print_profile(&profile);
}
