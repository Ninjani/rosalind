use crate::utils;
use crate::utils::Parseable;
use failure::Error;

/// Get the nth fibonacci number, given the series multiplies by k
pub fn fibonacci(n: u64, k: u64) -> u64 {
    let mut fib = [1u64, 1u64];
    for _ in 2..n {
        fib = [fib[1], fib[1] + k * fib[0]];
    }
    fib[1]
}

/// Rabbits and Recurrence Relations
///
/// Given: Positive integers n≤40 and k≤5.
///
/// Return: The total number of rabbit pairs that will be present after n months, if we begin with 1 pair and in each generation, every pair of reproduction-age rabbits produces a litter of k rabbit pairs (instead of only 1 pair).
pub fn rosalind_fib() -> Result<(), Error> {
    let contents = u64::parse_line(&utils::input_from_file("data/stronghold/rosalind_fib.txt"))?;
    let (n, k) = (contents[0], contents[1]);
    println!("{}", fibonacci(n, k));
    Ok(())
}
