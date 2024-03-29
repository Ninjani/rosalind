use anyhow::Error;
use ndarray::Array2;
use std::path::Path;

/// Edit Distance
///
/// Given: Two protein strings s and t in FASTA format (each of length at most 1000 aa).
///
/// Return: The edit distance dE(s,t).
pub fn rosalind_edit(filename: &Path) -> Result<usize, Error> {
    let fasta = utility::io::read_fasta_file(filename)?;
    let sequences: Vec<String> = fasta.values().map(|x| x.to_owned()).collect();
    let (string_1, string_2) = (&sequences[0], &sequences[1]);
    let result = get_edit_distance(string_1, string_2);
    println!("{}", result);
    Ok(result)
}

pub fn get_edit_distance(string_1: &str, string_2: &str) -> usize {
    get_edit_distances(string_1, string_2)[(string_1.len(), string_2.len())]
}

pub fn get_edit_distances(string_1: &str, string_2: &str) -> Array2<usize> {
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
    for i in 1..=m {
        for j in 1..=n {
            let cost = if string_1[i - 1] == string_2[j - 1] {
                0
            } else {
                1
            };
            distances[(i, j)] = (distances[(i - 1, j)] + 1)
                .min(distances[(i, j - 1)] + 1)
                .min(distances[(i - 1, j - 1)] + cost);
        }
    }
    distances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edit() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_edit")?;
        assert_eq!(
            rosalind_edit(&input_file)?,
            utility::io::input_from_file(&output_file)?.parse::<usize>()?
        );
        Ok(())
    }
}
