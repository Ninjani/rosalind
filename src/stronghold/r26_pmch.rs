use crate::utils;

/// Perfect Matchings and RNA Secondary Structures
///
/// Given: An RNA string s of length at most 80 bp having the same number of occurrences of 'A' as 'U' and the same number of occurrences of 'C' as 'G'.
///
/// Return: The total possible number of perfect matchings of basepair edges in the bonding graph of s.
pub fn rosalind_pmch() {
    let sequences = utils::read_fasta_file("data/stronghold/rosalind_pmch.txt");
    let (_, sequence) = sequences.iter().collect::<Vec<_>>()[0];
    let nucleotide_counts = utils::char_counter(sequence);
    println!(
        "{}",
        utils::factorial(nucleotide_counts[&'A']) * utils::factorial(nucleotide_counts[&'C'])
    );
}
