use anyhow::Error;
use num::BigUint;

use std::path::Path;
use utility::io::Parseable;

/// Introduction to Alternative Splicing
///
/// Given: Positive integers n and m with 0≤m≤n≤2000.
///
/// Return: The sum of combinations C(n,k) for all k satisfying m≤k≤n, modulo 1,000,000.
pub fn rosalind_aspc(filename: &Path) -> Result<BigUint, Error> {
    let nm = u64::parse_line(&utility::io::input_from_file(filename)?)?;
    let (n, m) = (nm[0], nm[1]);
    let result = (m..=n)
        .map(|k| utility::math::ncr(n, k) % BigUint::from(10u64.pow(6)))
        .sum::<BigUint>()
        % BigUint::from(10u64.pow(6));
    println!("{}", result);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aspc() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_aspc")?;
        assert_eq!(
            rosalind_aspc(&input_file)?,
            utility::io::input_from_file(&output_file)?.parse::<BigUint>()?
        );
        Ok(())
    }
}
