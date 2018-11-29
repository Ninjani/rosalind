use crate::utils;
use ndarray::Array2;

/// Creating a Distance Matrix
///
/// Given: A collection of n (n≤10) DNA strings s1,…,sn of equal length (at most 1 kbp). Strings are given in FASTA format.
///
/// Return: The matrix D corresponding to the p-distance dp on the given strings. As always, note that your answer is allowed an absolute error of 0.001.
pub fn rosalind_pdst() {
    let (headers, sequences) =
        utils::read_fasta_file_and_headers("data/stronghold/rosalind_pdst.txt");
    let mut distance_matrix = Array2::<f64>::zeros((headers.len(), headers.len()));
    for i in 0..headers.len() {
        for j in 0..headers.len() {
            if i != j {
                distance_matrix[(i, j)] =
                    p_distance(&sequences[&headers[i]], &sequences[&headers[j]]);
            }
        }
    }
    for row in distance_matrix.genrows() {
        utils::print_array(&row.to_vec());
    }
}

fn p_distance(string_1: &str, string_2: &str) -> f64 {
    let mut differing = 0usize;
    for (c1, c2) in string_1.chars().zip(string_2.chars()) {
        if c1 != c2 {
            differing += 1
        }
    }
    (differing as f64) / (string_1.len() as f64)
}
