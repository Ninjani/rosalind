use crate::utils;
use ndarray::Array2;

/// Edit Distance
///
/// Given: Two protein strings s and t in FASTA format (each of length at most 1000 aa).
///
/// Return: The edit distance dE(s,t).
pub fn rosalind_edit() {
    let fasta = utils::read_fasta_file("data/stronghold/rosalind_edit.txt");
    let sequences: Vec<String> = fasta.values().map(|x| x.to_owned()).collect();
    let (string_1, string_2) = (&sequences[0], &sequences[1]);
    println!(
        "{}",
        get_edit_distance(string_1, string_2)
    );
}

pub fn get_edit_distance(string_1: &str, string_2: &str) -> usize {
    let (string_1, string_2): (Vec<_>, Vec<_>) =
        (string_1.chars().collect(), string_2.chars().collect());
    let (m, n) = (string_1.len(), string_2.len());
    let mut distances = Array2::<usize>::zeros((m + 1, n + 1));
    for i in 0..=m {
        distances[(i, 0)] = i;
    }
    for j in 0..=n {
        distances[(0, j)] = j;
    }
    for j in 1..=n {
        for i in 1..=m {
            distances[(i, j)] = if string_1[i - 1] == string_2[j - 1] {
                distances[(i - 1, j - 1)]
            } else {
                (distances[(i - 1, j)] + 1)
                    .min(distances[(i, j - 1)] + 1)
                    .min(distances[(i - 1, j - 1)] + 1)
            };
        }
    }
    distances[(string_1.len(), string_2.len())]
}
