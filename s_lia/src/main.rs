use failure::Error;
use num::ToPrimitive;

use crate::utility;
use crate::utility::io::Parseable;

/// Independent Alleles
///
/// Given: Two positive integers k (k≤7) and N (N≤2k). In this problem, we begin with Tom,
/// who in the 0th generation has genotype Aa Bb. Tom has two children in the 1st generation,
/// each of whom has two children, and so on.
/// Each organism always mates with an organism having genotype Aa Bb.
///
/// Return: The probability that at least N Aa Bb organisms will belong to the k-th generation
/// of Tom's family tree (don't count the Aa Bb mates at each level).
/// Assume that Mendel's second law holds for the factors.
pub fn rosalind_lia(filename: &str) -> Result<f64, Error> {
    let input = utility::io::input_from_file(filename)?;
    let contents = u64::parse_line(&input)?;
    let k = contents[0];
    let n = contents[1];
    let total = 2u64.pow(k as u32);
    Ok((n..=total)
        .map(|i| {
            utility::math::ncr(total, i).to_f64().unwrap()
                * 0.25f64.powi(i as i32)
                * 0.75f64.powi((total - i) as i32)
        })
        .sum::<f64>())
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn lia() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_lia")?;
        assert_approx_eq!(
            rosalind_lia(&input_file)?,
            utility::io::input_from_file(&output_file)?.parse::<f64>()?,
            utility::testing::ROSALIND_FLOAT_ERROR_F64
        );
        Ok(())
    }
}
