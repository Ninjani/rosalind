use std::isize;

use failure::Error;
use ndarray::Array2;

use t_ba5e::{align, AlignmentParameters, read_scoring_matrix};
use utility;

/// Find a Highest-Scoring Fitting Alignment of Two Strings
///
/// Given: Two DNA strings v and w, where v has length at most 10000 and w has length at most 1000.
///
/// Return: The maximum score of a fitting alignment of v and w, followed by a fitting alignment
/// achieving this maximum score. Use the simple scoring method in which matches count +1 and both
/// the mismatch and indel penalties are equal to 1. (If multiple fitting alignments achieving
/// the maximum score exist, you may return any one.)
pub fn rosalind_ba5h(filename: &str) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let lines: Vec<_> = contents.split('\n').collect();
    let (_, amino_acids) = read_scoring_matrix(utility::io::PAM_FILE)?;
    let mut scoring_matrix = Array2::<isize>::zeros((amino_acids.len(), amino_acids.len()));
    scoring_matrix.fill(-1);
    scoring_matrix.diag_mut().fill(1);
    let parameters = AlignmentParameters::new(scoring_matrix, amino_acids, 1);
    let (score, aln_string_1, aln_string_2) = fitting_align(lines[0], lines[1], &parameters);
    println!("{}\n{}\n{}", score, aln_string_1, aln_string_2);
    Ok(())
}

pub fn fitting_alignment_backtrack(
    string_1: &str,
    string_2: &str,
    parameters: &AlignmentParameters,
) -> (Array2<isize>, Array2<usize>) {
    let chars_1: Vec<_> = string_1.chars().collect();
    let chars_2: Vec<_> = string_2.chars().collect();
    let mut scores = Array2::zeros((chars_1.len() + 1, chars_2.len() + 1));
    let mut backtrack = Array2::zeros((chars_1.len() + 1, chars_2.len() + 1));
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
            if j == chars_2.len() && max_value <= 0 {
                scores[(i, j)] = 0;
                backtrack[(i, j)] = 0;
            }
        }
    }
    (scores, backtrack)
}

fn fitting_align(
    string_1: &str,
    string_2: &str,
    parameters: &AlignmentParameters,
) -> (isize, String, String) {
    let (scores, backtrack) = fitting_alignment_backtrack(string_1, string_2, parameters);
    let (string_1, string_2): (Vec<_>, Vec<_>) =
        (string_1.chars().collect(), string_2.chars().collect());
    let mut n = 0;
    let mut max_score = isize::MIN;
    for i in 0..=string_1.len() {
        if scores[(i, string_2.len())] > max_score {
            max_score = scores[(i, string_2.len())];
            n = i;
        }
    }
    let (aln_1, aln_2) = align(&backtrack, &string_1, &string_2, n, string_2.len());
    (max_score, aln_1, aln_2)
}
