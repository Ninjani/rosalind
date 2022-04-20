use std::isize;

use anyhow::Error;
use ndarray::Array2;

use std::path::{Path, PathBuf};
use t_ba5e::{align, read_scoring_matrix, AlignmentParameters};

/// Find a Highest-Scoring Overlap Alignment of Two Strings
///
/// Given: Two protein strings v and w, each of length at most 1000.
///
/// Return: The score of an optimal overlap alignment of v and w, followed by an alignment of a
/// suffix v’ of v and a prefix w’ of w achieving this maximum score. Use an alignment score in
/// which matches count +1 and both the mismatch and indel penalties are 2. (If multiple overlap
/// alignments achieving the maximum score exist, you may return any one.)
pub fn rosalind_ba5i(filename: &Path) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let lines: Vec<_> = contents.split('\n').collect();
    let pam_file: PathBuf = [env!("CARGO_WORKSPACE_DIR"), utility::io::PAM_FILE]
        .iter()
        .collect();
    let (_, amino_acids) = read_scoring_matrix(&pam_file)?;
    let mut scoring_matrix = Array2::zeros((amino_acids.len(), amino_acids.len()));
    scoring_matrix.fill(-2);
    scoring_matrix.diag_mut().fill(1);
    let parameters = AlignmentParameters::new(scoring_matrix, amino_acids, 2);
    let (score, aln_string_1, aln_string_2) = overlap_align(lines[0], lines[1], &parameters);
    println!("{}\n{}\n{}", score, aln_string_1, aln_string_2);
    Ok(())
}

pub fn overlap_alignment_backtrack(
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
            if i == chars_1.len() && max_value <= 0 {
                scores[(i, j)] = 0;
                backtrack[(i, j)] = 0;
            }
        }
    }
    (scores, backtrack)
}

fn overlap_align(
    string_1: &str,
    string_2: &str,
    parameters: &AlignmentParameters,
) -> (isize, String, String) {
    let (scores, backtrack) = overlap_alignment_backtrack(string_1, string_2, parameters);
    let (string_1, string_2): (Vec<_>, Vec<_>) =
        (string_1.chars().collect(), string_2.chars().collect());
    let mut m = 0;
    let mut max_score = isize::MIN;
    for j in 0..=string_2.len() {
        if scores[(string_1.len(), j)] > max_score {
            max_score = scores[(string_1.len(), j)];
            m = j;
        }
    }
    let (aln_1, aln_2) = align(&backtrack, &string_1, &string_2, string_1.len(), m);
    (max_score, aln_1, aln_2)
}
