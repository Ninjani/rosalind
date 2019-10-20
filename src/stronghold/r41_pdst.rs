use failure::Error;
use ndarray::Array2;

use crate::utility;

/// Creating a Distance Matrix
///
/// Given: A collection of n (n≤10) DNA strings s1,…,sn of equal length (at most 1 kbp).
/// Strings are given in FASTA format.
///
/// Return: The matrix D corresponding to the p-distance dp on the given strings.
/// As always, note that your answer is allowed an absolute error of 0.001.
pub fn rosalind_pdst(filename: &str) -> Result<Array2<f64>, Error> {
    let (headers, sequences) = utility::io::read_fasta_file_and_headers(filename)?;
    let mut distance_matrix = Array2::<f64>::zeros((headers.len(), headers.len()));
    for i in 0..headers.len() {
        for j in 0..headers.len() {
            if i != j {
                distance_matrix[(i, j)] =
                    p_distance(&sequences[&headers[i]], &sequences[&headers[j]]);
            }
        }
    }
    for row in distance_matrix.genrows() {
        println!("{}", utility::io::format_array(&row.to_vec()));
    }
    Ok(distance_matrix)
}

fn p_distance(string_1: &str, string_2: &str) -> f64 {
    let mut differing = 0usize;
    for (c1, c2) in string_1.chars().zip(string_2.chars()) {
        if c1 != c2 {
            differing += 1
        }
    }
    (differing as f64) / (string_1.len() as f64)
}

#[cfg(test)]
mod tests {
    use crate::utility::io::Parseable;

    use super::*;

    #[test]
    fn pdst() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_pdst")?;
        let result = rosalind_pdst(&input_file)?;
        let output = Array2::from_shape_vec(
            (result.shape()[0], result.shape()[1]),
            utility::io::input_from_file(&output_file)?
                .split('\n')
                .flat_map(|line| f64::parse_line(line).unwrap().into_iter())
                .collect(),
        )?;
        assert!((result - output)
            .iter()
            .all(|i| *i < utility::testing::ROSALIND_FLOAT_ERROR_F64));
        Ok(())
    }
}
