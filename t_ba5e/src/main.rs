use std::collections::HashMap;

use failure::Error;
use ndarray::{Array, Array2};

use crate::utility;

/// Find a Highest-Scoring Alignment of Two Strings
///
/// Given: Two amino acid strings.
///
/// Return: The maximum alignment score of these strings followed by an alignment achieving this
/// maximum score. Use the BLOSUM62 scoring matrix and indel penalty σ = 5.
/// (If multiple alignments achieving the maximum score exist, you may return any one.)
pub fn rosalind_ba5e() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba5e.txt")?;
    let lines: Vec<_> = contents.split('\n').collect();
    let (scoring_matrix, amino_acids) = read_scoring_matrix("data/blosum62.txt")?;
    let parameters = AlignmentParameters::new(scoring_matrix, amino_acids, 5);
    let (score, aln_string_1, aln_string_2) = global_align(lines[0], lines[1], &parameters);
    println!("{}\n{}\n{}", score, aln_string_1, aln_string_2);
    Ok(())
}

pub fn read_scoring_matrix(filename: &str) -> Result<(Array2<isize>, Vec<char>), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let mut lines = contents.split('\n');
    let amino_acids: Vec<_> = lines
        .next()
        .unwrap()
        .trim_start()
        .trim_end()
        .split_whitespace()
        .map(|s| s.chars().nth(0).unwrap())
        .collect();
    let mut scoring_matrix = Array2::zeros((amino_acids.len(), amino_acids.len()));
    for (i, line) in lines.enumerate() {
        let parts: Vec<_> = line.split_whitespace().collect();
        scoring_matrix.row_mut(i).assign(&Array::from_vec(
            parts[1..]
                .iter()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<_>>(),
        ));
    }
    Ok((scoring_matrix, amino_acids))
}

pub struct AlignmentParameters {
    pub scoring_matrix: Array2<isize>,
    pub amino_acid_order: HashMap<char, usize>,
    pub gap_penalty: isize,
}

impl AlignmentParameters {
    pub fn new(scoring_matrix: Array2<isize>, amino_acids: Vec<char>, gap_penalty: isize) -> Self {
        let amino_acid_order: HashMap<_, _> = amino_acids
            .into_iter()
            .enumerate()
            .map(|(i, a)| (a, i))
            .collect();
        AlignmentParameters {
            scoring_matrix,
            amino_acid_order,
            gap_penalty,
        }
    }
}

pub fn global_alignment_backtrack(
    string_1: &str,
    string_2: &str,
    parameters: &AlignmentParameters,
) -> (Array2<isize>, Array2<usize>) {
    let chars_1: Vec<_> = string_1.chars().collect();
    let chars_2: Vec<_> = string_2.chars().collect();
    let mut scores = Array2::zeros((chars_1.len() + 1, chars_2.len() + 1));
    let mut backtrack = Array2::zeros((chars_1.len() + 1, chars_2.len() + 1));
    for i in 1..chars_1.len() {
        scores[(i, 0)] = scores[(i - 1, 0)] - parameters.gap_penalty;
        backtrack[(i, 0)] = 1;
    }
    for j in 1..=chars_2.len() {
        scores[(0, j)] = scores[(0, j - 1)] - parameters.gap_penalty;
        backtrack[(0, j)] = 2;
    }
    for i in 1..=chars_1.len() {
        for j in 1..=chars_2.len() {
            let values: Vec<isize> = vec![
                (scores[(i - 1, j)] - parameters.gap_penalty),
                (scores[(i, j - 1)] - parameters.gap_penalty),
                (scores[(i - 1, j - 1)]
                    + parameters.scoring_matrix[(
                    parameters.amino_acid_order[&chars_1[i - 1]],
                    parameters.amino_acid_order[&chars_2[j - 1]],
                )]),
            ];
            let (max_index, max_value) = values
                .into_iter()
                .enumerate()
                .max_by(|a, b| a.1.cmp(&b.1))
                .unwrap();
            scores[(i, j)] = max_value;
            backtrack[(i, j)] = max_index + 1;
        }
    }
    (scores, backtrack)
}

pub fn align(
    backtrack: &Array2<usize>,
    string_1: &[char],
    string_2: &[char],
    n: usize,
    m: usize,
) -> (String, String) {
    let (mut aln_1, mut aln_2) = (String::new(), String::new());
    let (mut n, mut m) = (n, m);
    while !(n == 0 && m == 0) {
        if backtrack[(n, m)] == 0 {
            break;
        } else if m == 0 || backtrack[(n, m)] == 1 {
            n -= 1;
            aln_1.push(string_1[n]);
            aln_2.push('-');
        } else if n == 0 || backtrack[(n, m)] == 2 {
            m -= 1;
            aln_1.push('-');
            aln_2.push(string_2[m]);
        } else {
            n -= 1;
            m -= 1;
            aln_1.push(string_1[n]);
            aln_2.push(string_2[m]);
        }
    }
    (aln_1.chars().rev().collect(), aln_2.chars().rev().collect())
}

fn global_align(
    string_1: &str,
    string_2: &str,
    parameters: &AlignmentParameters,
) -> (isize, String, String) {
    let (scores, backtrack) = global_alignment_backtrack(string_1, string_2, parameters);
    let (string_1, string_2): (Vec<_>, Vec<_>) =
        (string_1.chars().collect(), string_2.chars().collect());
    let (aln_1, aln_2) = align(
        &backtrack,
        &string_1,
        &string_2,
        string_1.len(),
        string_2.len(),
    );
    (scores[(string_1.len(), string_2.len())], aln_1, aln_2)
}
