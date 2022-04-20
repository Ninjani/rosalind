use anyhow::Error;

use std::path::Path;
use utility::io::Parseable;

/// Calculating Expected Offspring
///
/// Given: Six nonnegative integers, each of which does not exceed 20,000.
/// The integers correspond to the number of couples in a population possessing
/// each genotype pairing for a given factor.
/// In order, the six given integers represent the number of couples having the following genotypes:
///
///    AA-AA
///
///    AA-Aa
///
///    AA-aa
///
///    Aa-Aa
///
///    Aa-aa
///
///    aa-aa
///
/// Return: The expected number of offspring displaying the dominant phenotype
/// in the next generation, under the assumption that every couple has exactly two offspring.
pub fn rosalind_iev(filename: &Path) -> Result<f64, Error> {
    let input = utility::io::input_from_file(filename)?;
    let contents = f64::parse_line(&input)?;
    let percentages = [1., 1., 1., 0.75, 0.5, 0.];
    let output = contents
        .iter()
        .enumerate()
        .map(|(i, c)| percentages[i] * c)
        .sum::<f64>()
        * 2.;
    println!("{}", output);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn iev() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_iev")?;
        assert_approx_eq!(
            rosalind_iev(&input_file)?,
            utility::io::input_from_file(&output_file)?.parse::<f64>()?,
            utility::testing::ROSALIND_FLOAT_ERROR_F64
        );
        Ok(())
    }
}
