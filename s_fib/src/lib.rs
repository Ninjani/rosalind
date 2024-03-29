use anyhow::Error;

use std::path::Path;
use utility::io::Parseable;

/// Rabbits and Recurrence Relations
///
/// Given: Positive integers n≤40 and k≤5.
///
/// Return: The total number of rabbit pairs that will be present after n months, if we begin with 1 pair and in each generation, every pair of reproduction-age rabbits produces a litter of k rabbit pairs (instead of only 1 pair).
pub fn rosalind_fib(filename: &Path) -> Result<u64, Error> {
    let input = utility::io::input_from_file(filename)?;
    let contents = u64::parse_line(&input)?;
    let (n, k) = (contents[0], contents[1]);
    let output = fibonacci(n, k);
    println!("{}", output);
    Ok(output)
}

/// Get the nth fibonacci number, given the series multiplies by k
pub fn fibonacci(n: u64, k: u64) -> u64 {
    let mut fib = [1u64, 1u64];
    for _ in 2..n {
        fib = [fib[1], fib[1] + k * fib[0]];
    }
    fib[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_fib")?;
        assert_eq!(
            rosalind_fib(&input_file)?,
            utility::io::input_from_file(&output_file)?.parse::<u64>()?
        );
        Ok(())
    }
}
