use crate::utils;
use ndarray::Array2;
/// Finding a Shared Spliced Motif
///
/// Given: Two DNA strings s and t (each having length at most 1 kbp) in FASTA format.
///
/// Return: A longest common subsequence of s and t. (If more than one solution exists, you may return any one.)
pub fn rosalind_lcsq() {
    let fasta = utils::read_fasta_file("data/stronghold/rosalind_lcsq.txt");
    let sequences: Vec<String> = fasta.values().map(|x| x.to_owned()).collect();
    println!(
        "{}",
        longest_common_subsequence(&sequences[0], &sequences[1])
    );
}

pub fn longest_common_subsequence(string_1: &str, string_2: &str) -> String {
    let sequence_1: Vec<_> = string_1.chars().collect();
    let sequence_2: Vec<_> = string_2.chars().collect();
    let c_matrix = lcs_make_c_matrix(&sequence_1, &sequence_2);
    lcs_backtrack(
        &c_matrix,
        &sequence_1,
        &sequence_2,
        sequence_1.len(),
        sequence_2.len(),
    )
}

fn lcs_make_c_matrix(sequence_1: &[char], sequence_2: &[char]) -> Array2<usize> {
    let (m, n) = (sequence_1.len(), sequence_2.len());
    let mut c_matrix = Array2::<usize>::zeros((m + 1, n + 1));
    for i in 1..=m {
        for j in 1..=n {
            if sequence_1[i - 1] == sequence_2[j - 1] {
                c_matrix[(i, j)] = c_matrix[(i - 1, j - 1)] + 1;
            } else {
                c_matrix[(i, j)] = c_matrix[(i - 1, j)].max(c_matrix[(i, j - 1)]);
            }
        }
    }
    c_matrix
}

fn lcs_backtrack(
    c_matrix: &Array2<usize>,
    sequence_1: &[char],
    sequence_2: &[char],
    i: usize,
    j: usize,
) -> String {
    if i == 0 || j == 0 {
        "".into()
    } else if sequence_1[i - 1] == sequence_2[j - 1] {
        format!(
            "{}{}",
            lcs_backtrack(c_matrix, sequence_1, sequence_2, i - 1, j - 1),
            sequence_1[i - 1]
        )
    } else if c_matrix[(i, j - 1)] > c_matrix[(i - 1, j)] {
        lcs_backtrack(c_matrix, sequence_1, sequence_2, i, j - 1)
    } else {
        lcs_backtrack(c_matrix, sequence_1, sequence_2, i - 1, j)
    }
}
