#[macro_use]
extern crate ndarray;
use std::collections::HashMap;

use anyhow::Error;
use ndarray::{Array2, Array3};

use std::path::{Path, PathBuf};
use t_ba5e::read_scoring_matrix;

/// Align Two Strings Using Affine Gap Penalties
///
/// Given: Two amino acid strings v and w (each of length at most 100).
///
/// Return: The maximum alignment score between v and w, followed by an alignment of v and w
/// achieving this maximum score. Use the BLOSUM62 scoring matrix, a gap opening penalty of 11,
/// and a gap extension penalty of 1.
pub fn rosalind_ba5j(filename: &Path) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let lines: Vec<_> = contents.split('\n').collect();
    let blosum_file: PathBuf = [env!("CARGO_WORKSPACE_DIR"), utility::io::BLOSUM_FILE]
        .iter()
        .collect();
    let (scoring_matrix, amino_acids) = read_scoring_matrix(&blosum_file)?;
    let parameters = AlignmentParameters::new(scoring_matrix, amino_acids, 11, 1);
    let (score, aln_string_1, aln_string_2) =
        affine_gap_penalties_align(lines[0], lines[1], &parameters);
    println!("{}\n{}\n{}", score, aln_string_1, aln_string_2);
    Ok(())
}

pub struct AlignmentParameters {
    pub scoring_matrix: Array2<isize>,
    pub amino_acid_order: HashMap<char, usize>,
    pub gap_open_penalty: isize,
    pub gap_extension_penalty: isize,
}

impl AlignmentParameters {
    pub fn new(
        scoring_matrix: Array2<isize>,
        amino_acids: Vec<char>,
        gap_open_penalty: isize,
        gap_extension_penalty: isize,
    ) -> Self {
        let amino_acid_order: HashMap<_, _> = amino_acids
            .into_iter()
            .enumerate()
            .map(|(i, a)| (a, i))
            .collect();
        AlignmentParameters {
            scoring_matrix,
            amino_acid_order,
            gap_open_penalty,
            gap_extension_penalty,
        }
    }
}

fn get_max_index_max_value<T: Ord + Copy>(values: &[T]) -> (usize, T) {
    let (index, value) = values
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap();
    (index, *value)
}

pub fn affine_gap_penalties_alignment_backtrack(
    string_1: &str,
    string_2: &str,
    parameters: &AlignmentParameters,
) -> (Array3<isize>, Array3<usize>) {
    let chars_1: Vec<_> = string_1.chars().collect();
    let chars_2: Vec<_> = string_2.chars().collect();
    let mut scores = Array3::<isize>::zeros((chars_1.len() + 1, chars_2.len() + 1, 3));
    let mut backtrack = Array3::<usize>::zeros((chars_1.len() + 1, chars_2.len() + 1, 3));
    for i in 1..chars_1.len() {
        scores[(i, 0, 0)] =
            -parameters.gap_open_penalty - (i as isize - 1) * parameters.gap_extension_penalty;
        scores[(i, 0, 1)] =
            -parameters.gap_open_penalty - (i as isize - 1) * parameters.gap_extension_penalty;
        scores[(i, 0, 2)] = ::std::isize::MIN + parameters.gap_open_penalty;
        backtrack.slice_mut(s![i, 0, ..]).fill(0);
    }
    for j in 1..=chars_2.len() {
        scores[(0, j, 0)] = ::std::isize::MIN + parameters.gap_open_penalty;
        scores[(0, j, 1)] =
            -parameters.gap_open_penalty - (j as isize - 1) * parameters.gap_extension_penalty;
        scores[(0, j, 2)] =
            -parameters.gap_open_penalty - (j as isize - 1) * parameters.gap_extension_penalty;
        backtrack.slice_mut(s![0, j, ..]).fill(1);
    }
    for i in 1..=chars_1.len() {
        for j in 1..=chars_2.len() {
            let (max_index_lower, max_value_lower) = get_max_index_max_value(&[
                scores[(i - 1, j, 0)] - parameters.gap_extension_penalty, // 0
                scores[(i - 1, j, 1)] - parameters.gap_open_penalty,      // 1
            ]);
            scores[(i, j, 0)] = max_value_lower;
            backtrack[(i, j, 0)] = max_index_lower;

            let (max_index_upper, max_value_upper) = get_max_index_max_value(&[
                scores[(i, j - 1, 1)] - parameters.gap_open_penalty, // 1
                scores[(i, j - 1, 2)] - parameters.gap_extension_penalty, // 2
            ]);
            scores[(i, j, 2)] = max_value_upper;
            backtrack[(i, j, 2)] = max_index_upper + 1;

            let (max_index, max_value) = get_max_index_max_value(&[
                scores[(i, j, 0)], // 0
                (scores[(i - 1, j - 1, 1)]
                    + parameters.scoring_matrix[(
                        parameters.amino_acid_order[&chars_1[i - 1]],
                        parameters.amino_acid_order[&chars_2[j - 1]],
                    )]), // 1
                scores[(i, j, 2)], // 2
            ]);
            scores[(i, j, 1)] = max_value;
            backtrack[(i, j, 1)] = max_index;
        }
    }
    (scores, backtrack)
}

pub fn align(
    start_direction: usize,
    backtrack: &Array3<usize>,
    string_1: &[char],
    string_2: &[char],
    n: usize,
    m: usize,
) -> (String, String) {
    let (mut aln_1, mut aln_2) = (String::new(), String::new());
    let (mut n, mut m) = (n, m);
    let mut direction = start_direction;
    while !(n == 0 && m == 0) {
        if m == 0 {
            n -= 1;
            aln_1.push(string_1[n]);
            aln_2.push('-');
        } else if n == 0 {
            m -= 1;
            aln_1.push('-');
            aln_2.push(string_2[m]);
        } else {
            match direction {
                0 => {
                    direction = backtrack[(n, m, 0)];
                    n -= 1;
                    aln_1.push(string_1[n]);
                    aln_2.push('-');
                }
                1 => {
                    direction = backtrack[(n, m, 1)];
                    if direction == 1 {
                        n -= 1;
                        m -= 1;
                        aln_1.push(string_1[n]);
                        aln_2.push(string_2[m]);
                    }
                }
                2 => {
                    direction = backtrack[(n, m, 2)];
                    m -= 1;
                    aln_1.push('-');
                    aln_2.push(string_2[m]);
                }
                _ => panic!("undefined direction"),
            }
        }
    }
    (aln_1.chars().rev().collect(), aln_2.chars().rev().collect())
}

fn affine_gap_penalties_align(
    string_1: &str,
    string_2: &str,
    parameters: &AlignmentParameters,
) -> (isize, String, String) {
    let (scores, backtrack) =
        affine_gap_penalties_alignment_backtrack(string_1, string_2, parameters);
    let (string_1, string_2): (Vec<_>, Vec<_>) =
        (string_1.chars().collect(), string_2.chars().collect());
    let (n, m) = (string_1.len(), string_2.len());
    let (best_direction, best_score) =
        get_max_index_max_value(&[scores[(n, m, 0)], scores[(n, m, 1)], scores[(n, m, 2)]]);
    let (aln_1, aln_2) = align(
        best_direction,
        &backtrack,
        &string_1,
        &string_2,
        string_1.len(),
        string_2.len(),
    );
    (best_score, aln_1, aln_2)
}
