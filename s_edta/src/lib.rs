use anyhow::Error;
use ndarray::Array2;

use s_edit::get_edit_distances;
use std::path::Path;

/// WIP
/// Edit Distance Alignment
///
/// Given: Two protein strings s and t in FASTA format (with each string having length at most 1000 aa).
///
/// Return: The edit distance dE(s,t) followed by two augmented strings s′ and t′
/// representing an optimal alignment of s and t.
pub fn rosalind_edta(filename: &Path) -> Result<(usize, String, String), Error> {
    let fasta = utility::io::read_fasta_file(filename)?;
    let sequences: Vec<String> = fasta.values().map(|x| x.to_owned()).collect();
    let (string_1, string_2) = (&sequences[0], &sequences[1]);
    let distances = get_edit_distances(string_1, string_2);
    let (aln_1, aln_2) = backtrack(string_1, string_2, &distances);
    let edit_distance = distances[(string_1.len(), string_2.len())];
    println!("{}\n{}\n{}", edit_distance, aln_1, aln_2);
    Ok((edit_distance, aln_1, aln_2))
}

fn backtrack(string_1: &str, string_2: &str, distances: &Array2<usize>) -> (String, String) {
    let (string_1, string_2): (Vec<_>, Vec<_>) =
        (string_1.chars().collect(), string_2.chars().collect());
    let (mut m, mut n) = (string_1.len(), string_2.len());
    let (mut aln_1, mut aln_2) = (Vec::new(), Vec::new());

    while m != 0 && n != 0 {
        let cost = if string_1[m - 1] == string_2[n - 1] {
            0
        } else {
            1
        };
        if distances[(m, n)] == distances[(m - 1, n - 1)] + cost {
            aln_1.insert(0, string_1[m - 1]);
            aln_2.insert(0, string_2[n - 1]);
            m -= 1;
            n -= 1;
        } else if m > 0 && distances[(m, n)] == distances[(m - 1, n)] + 1 {
            aln_1.insert(0, string_1[m - 1]);
            aln_2.insert(0, '-');
            m -= 1;
        } else {
            aln_1.insert(0, '-');
            aln_2.insert(0, string_2[n - 1]);
            n -= 1;
        }
    }
    (aln_1.into_iter().collect(), aln_2.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edta() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_edta")?;
        let (edit_distance, aln_1, aln_2) = rosalind_edta(&input_file)?;
        let output = utility::io::input_from_file(&output_file)?;
        let mut output_lines = output.split('\n');
        assert_eq!(
            edit_distance,
            output_lines.next().unwrap().parse::<usize>()?
        );
        assert_eq!(&aln_1, output_lines.next().unwrap());
        assert_eq!(&aln_2, output_lines.next().unwrap());
        Ok(())
    }
}
