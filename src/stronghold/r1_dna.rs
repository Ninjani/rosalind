use crate::utils;

/// Counting DNA Nucleotides
///
/// Given: A DNA string s of length at most 1000 nt.
///
/// Return: Four integers (separated by spaces) counting the respective number of times that the symbols 'A', 'C', 'G', and 'T' occur in s.
pub fn rosalind_dna() {
    let dna = utils::input_from_file("data/stronghold/rosalind_dna.txt");
    let counter = utils::char_counter(&dna);
    let counts = "ACGT"
        .chars()
        .map(|c| *counter.get(&c).unwrap_or(&0usize))
        .collect::<Vec<usize>>();
    utils::print_array(&counts);
}
