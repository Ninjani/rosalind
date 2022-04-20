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
    loop {
        if m == 0 && n == 0 {
            break;
        } else if n == 0 {
            m -= 1;
            aln_1.push(string_1[m]);
            aln_2.push('-')
        } else if m == 0 {
            n -= 1;
            aln_1.push('-');
            aln_2.push(string_2[n]);
        } else {
            let indices = [(m - 1, n - 1), (m - 1, n), (m, n - 1)];
            let (min_index, min_distance) = indices
                .iter()
                .enumerate()
                .map(|(i, x)| (i, distances[*x]))
                .min_by(|a, b| a.1.cmp(&b.1))
                .unwrap();
            println!(
                "{} {} {:?} {} {} {}",
                m,
                n,
                indices[min_index],
                min_distance,
                string_1[m - 1],
                string_2[n - 1]
            );
            if indices[min_index].0 == m - 1 {
                aln_1.push(string_1[m - 1]);
            } else {
                aln_1.push('-');
            }
            if indices[min_index].1 == n - 1 {
                aln_2.push(string_2[n - 1]);
            } else {
                aln_2.push('-');
            }
            m = indices[min_index].0;
            n = indices[min_index].1;
        }
    }
    (
        aln_1.into_iter().rev().collect(),
        aln_2.into_iter().rev().collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
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
