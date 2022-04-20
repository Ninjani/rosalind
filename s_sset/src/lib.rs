use anyhow::Error;

use std::path::Path;

/// Counting Subsets
///
/// Given: A positive integer n (n≤1000).
///
/// Return: The total number of subsets of {1,2,…,n} modulo 1,000,000.
pub fn rosalind_sset(filename: &Path) -> Result<usize, Error> {
    let n = utility::io::input_from_file(filename)?.parse::<usize>()?;
    let num_subsets = pow_mod(2, n, 10usize.pow(6));
    println!("{}", num_subsets);
    Ok(num_subsets)
}

/// Modular Exponentiation
/// function modular_pow(base, exponent, modulus)
///    if modulus = 1 then return 0
///    Assert :: (modulus - 1) * (modulus - 1) does not overflow base
///    result := 1
///    base := base mod modulus
///    while exponent > 0
///        if (exponent mod 2 == 1):
///           result := (result * base) mod modulus
///        exponent := exponent >> 1
///        base := (base * base) mod modulus
///    return result
///
fn pow_mod(base: usize, exponent: usize, modulus: usize) -> usize {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    let (mut base, mut exponent) = (base % modulus, exponent);
    while exponent > 0 {
        if exponent % (1 + 1) == 1 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sset() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_sset")?;
        assert_eq!(
            rosalind_sset(&input_file)?,
            utility::io::input_from_file(&output_file)?.parse::<usize>()?
        );
        Ok(())
    }
}
