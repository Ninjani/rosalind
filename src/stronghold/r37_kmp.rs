use crate::utils;
use std::iter::repeat;

/// Speeding Up Motif Finding
///
/// Given: A DNA string s (of length at most 100 kbp) in FASTA format.
///
/// Return: The failure array of s.
pub fn rosalind_kmp() {
    let dna = utils::read_fasta_file("data/stronghold/rosalind_kmp.txt");
    for (_, sequence) in dna {
        utils::print_array(&compute_failure_array(&sequence));
    }
}

fn compute_failure_array(string: &str) -> Vec<isize> {
    let n = string.len();
    let characters: Vec<_> = string.chars().collect();
    let mut failure_array: Vec<isize> = repeat(-1).take(n + 1).collect();
    let mut j = -1isize;
    for i in 1..=n {
        while (j >= 0) && (characters[i - 1] != characters[(j as usize)]) {
            j = failure_array[j as usize];
        }
        j += 1;
        failure_array[i] = j;
    }
    failure_array[1..].to_vec()
}
