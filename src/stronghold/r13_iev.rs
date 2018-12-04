use crate::utils;
use crate::utils::Parseable;
use failure::Error;

/// Calculating Expected Offspring
///
/// Given: Six nonnegative integers, each of which does not exceed 20,000. The integers correspond to the number of couples in a population possessing each genotype pairing for a given factor. In order, the six given integers represent the number of couples having the following genotypes:
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
/// Return: The expected number of offspring displaying the dominant phenotype in the next generation, under the assumption that every couple has exactly two offspring.
pub fn rosalind_iev() -> Result<(), Error> {
    let contents =
        f64::parse_line(&utils::input_from_file("data/stronghold/rosalind_iev.txt"))?;
    let percentages = [1., 1., 1., 0.75, 0.5, 0.];
    println!(
        "{}",
        contents
            .iter()
            .enumerate()
            .map(|(i, c)| percentages[i] * c)
            .sum::<f64>()
            * 2.
    );
    Ok(())
}
