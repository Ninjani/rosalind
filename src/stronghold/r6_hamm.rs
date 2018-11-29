use crate::utils;

/// Get hamming distance between two equal-length strings
pub fn hamming(string_1: &str, string_2: &str) -> usize {
    string_1
        .chars()
        .zip(string_2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

/// Counting Point Mutations
///
/// Given: Two DNA strings s and t of equal length (not exceeding 1 kbp).
///
/// Return: The Hamming distance d_H(s,t).
pub fn rosalind_hamm() {
    let contents = utils::input_from_file("data/stronghold/rosalind_hamm.txt");
    let sequences = contents.split('\n').collect::<Vec<&str>>();
    let (sequence_1, sequence_2) = (sequences[0], sequences[1]);
    println!("{}", hamming(sequence_1, sequence_2));
}
