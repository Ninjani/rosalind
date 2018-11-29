use crate::utils;
use std::collections::HashMap;

/// Get the reverse complement of a DNA string
pub fn reverse_complement(dna: &str) -> String {
    let nucleotide_map: HashMap<_, _> = "ATCG".chars().zip("TAGC".chars()).collect();
    dna.to_ascii_uppercase()
        .chars()
        .rev()
        .map(|c| &nucleotide_map[&c])
        .collect::<String>()
}

/// Complementing a Strand of DNA
///
/// Given: A DNA string s of length at most 1000 bp.
///
/// Return: The reverse complement s^c of s
pub fn rosalind_revc() {
    let dna = utils::input_from_file("data/stronghold/rosalind_revc.txt");
    println!("{}", reverse_complement(&dna));
}
