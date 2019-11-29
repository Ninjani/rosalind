use failure::Error;

use utility;
use utility::io::Parseable;

/// Mendel's First Law
///
/// Given: Three positive integers k, m, and n, representing a population containing k+m+n organisms:
/// k individuals are homozygous dominant for a factor, m are heterozygous, and n are homozygous recessive.
///
/// Return: The probability that two randomly selected mating organisms will produce an individual
/// possessing a dominant allele (and thus displaying the dominant phenotype).
/// Assume that any two organisms can mate.
pub fn rosalind_iprb(filename: &str) -> Result<f64, Error> {
    let contents = u64::parse_line(&utility::io::input_from_file(filename)?)?;
    let (k, m, n) = (contents[0], contents[1], contents[2]);
    let output = dominant_probability(k, m, n);
    println!("{}", output);
    Ok(output)
}

/// Choosing 2 individuals from 2 different populations
fn choose_2_different(x1: u64, x2: u64, total: u64) -> f64 {
    let x1 = x1 as f64;
    let x2 = x2 as f64;
    let total = total as f64;
    x1 / total * x2 / (total - 1.) + x2 / total * x1 / (total - 1.)
}

/// Choosing 2 individuals from the same population
fn choose_2_same(x1: u64, total: u64) -> f64 {
    let x1 = x1 as f64;
    let total = total as f64;
    x1 / total * (x1 - 1.) / (total - 1.)
}

/// Probability of dominant allele
fn dominant_probability(k: u64, m: u64, n: u64) -> f64 {
    let total = k + m + n;
    choose_2_same(k, total)
        + choose_2_different(k, m, total)
        + choose_2_different(k, n, total)
        + 0.75 * choose_2_same(m, total)
        + 0.5 * choose_2_different(m, n, total)
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn iprb() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_iprb")?;
        assert_approx_eq!(
            rosalind_iprb(&input_file)?,
            utility::io::input_from_file(&output_file)?.parse::<f64>()?,
            utility::testing::ROSALIND_FLOAT_ERROR_F64
        );
        Ok(())
    }
}
