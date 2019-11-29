use failure::Error;
use num::bigint::BigUint;

use utility;
use utility::io::Parseable;

/// Partial Permutations
///
/// Given: Positive integers n and k such that 100≥n>0 and 10≥k>0.
///
/// Return: The total number of partial permutations P(n,k), modulo 1,000,000.
pub fn rosalind_pper(filename: &str) -> Result<BigUint, Error> {
    let contents = u64::parse_line(&utility::io::input_from_file(filename)?)?;
    let (n, k) = (contents[0], contents[1]);
    let result = (utility::math::ncr(n, k) * utility::math::factorial(k as usize))
        % BigUint::from(1_000_000u64);
    println!("{}", result);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pper() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_pper")?;
        let output = utility::io::input_from_file(&output_file)?.parse::<BigUint>()?;
        assert_eq!(rosalind_pper(&input_file)?, output);
        Ok(())
    }
}
