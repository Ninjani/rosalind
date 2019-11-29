use std::collections::HashMap;

use failure::Error;

use utility;
use utility::io::Parseable;

/// Introduction to Random Strings
///
/// Given: A DNA string s of length at most 100 bp and an array A
/// containing at most 20 numbers between 0 and 1.
///
/// Return: An array B having the same length as A in which B[k] represents the common logarithm
/// of the probability that a random string constructed with the GC-content found in A[k] will
/// match s exactly.
pub fn rosalind_prob(filename: &str) -> Result<Vec<f64>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let parts: Vec<_> = input.split('\n').collect();
    let sequence = parts[0];
    let gc_contents = f64::parse_line(parts[1])?;
    let mut probabilities = Vec::new();
    for gc_content in gc_contents {
        let nucleotide_probs = nucleotide_probs_from_gc_content(gc_content);
        probabilities.push(
            sequence
                .chars()
                .map(|c| nucleotide_probs[&c].log10())
                .sum::<f64>(),
        );
    }
    println!("{}", utility::io::format_array(&probabilities));
    Ok(probabilities)
}

/// Get expected probabilities of each nucleotide from the GC content
pub fn nucleotide_probs_from_gc_content(gc_content: f64) -> HashMap<char, f64> {
    let gc = gc_content / 2.;
    let at = (1. - gc_content) / 2.;
    "ACGT".chars().zip(vec![at, gc, gc, at]).collect()
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn prob() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_prob")?;
        let output = f64::parse_line(&utility::io::input_from_file(&output_file)?)?;
        let result = rosalind_prob(&input_file)?;
        result
            .into_iter()
            .zip(output.into_iter())
            .for_each(|(x, y)| assert_approx_eq!(x, y, utility::testing::ROSALIND_FLOAT_ERROR_F64));
        Ok(())
    }
}
