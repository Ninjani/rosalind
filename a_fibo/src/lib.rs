use failure::Error;

use s_fib::fibonacci;
use utility;

/// Fibonacci Numbers
///
/// Given: A positive integer n≤25
///
/// Return: The value of F_n.
pub fn rosalind_fibo(filename: &str) -> Result<u64, Error> {
    let input = utility::io::input_from_file(filename)?;
    let n = input.parse::<u64>()?;
    Ok(fibonacci(n, 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibo() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_fibo")?;
        let output = utility::io::input_from_file(&output_file)?.parse::<u64>()?;
        assert_eq!(rosalind_fibo(&input_file)?, output);
        Ok(())
    }
}
