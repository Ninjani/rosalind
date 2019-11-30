use std::isize;

use failure::Error;
use ndarray::Array2;

use t_ba5e::{align, AlignmentParameters, read_scoring_matrix};
use utility;

/// Find a Highest-Scoring Local Alignment of Two Strings
///
/// Given: Two amino acid strings.
///
/// Return: The maximum score of a local alignment of the strings, followed by a local alignment of
/// these strings achieving the maximum score. Use the PAM250 scoring matrix and indel penalty σ = 5.
/// (If multiple local alignments achieving the maximum score exist, you may return any one.)
pub fn rosalind_ba5f(filename: &str) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let lines: Vec<_> = contents.split('\n').collect();
    let (scoring_matrix, amino_acids) = read_scoring_matrix("data/pam250.txt")?;
    let parameters = AlignmentParameters::new(scoring_matrix, amino_acids, 5);
    let (score, aln_string_1, aln_string_2) = local_align(lines[0], lines[1], &parameters);
    println!("{}\n{}\n{}", score, aln_string_1, aln_string_2);
    Ok(())
}

pub fn local_alignment_backtrack(
    string_1: &str,
    string_2: &str,
    parameters: &AlignmentParameters,
) -> (Array2<isize>, Array2<usize>) {
    let chars_1: Vec<_> = string_1.chars().collect();
    let chars_2: Vec<_> = string_2.chars().collect();
    let mut scores = Array2::zeros((chars_1.len() + 1, chars_2.len() + 1));
    let mut backtrack = Array2::zeros((chars_1.len() + 1, chars_2.len() + 1));
    for i in 1..=chars_1.len() {
        for j in 1..=chars_2.len() {
            let values: Vec<isize> = vec![
                0,
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
            backtrack[(i, j)] = max_index;
        }
    }
    (scores, backtrack)
}

fn local_align(
    string_1: &str,
    string_2: &str,
    parameters: &AlignmentParameters,
) -> (isize, String, String) {
    let (scores, backtrack) = local_alignment_backtrack(string_1, string_2, parameters);
    let (string_1, string_2): (Vec<_>, Vec<_>) =
        (string_1.chars().collect(), string_2.chars().collect());
    let (mut n, mut m) = (0, 0);
    let mut max_score = isize::MIN;
    for i in 0..=string_1.len() {
        for j in 0..=string_2.len() {
            if scores[(i, j)] > max_score {
                max_score = scores[(i, j)];
                n = i;
                m = j;
            }
        }
    }
    let (aln_1, aln_2) = align(&backtrack, &string_1, &string_2, n, m);
    (max_score, aln_1, aln_2)
}
