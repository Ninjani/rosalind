use crate::utils;
use crate::utils::Parseable;
use num::bigint::BigUint;

/// Partial Permutations
///
/// Given: Positive integers n and k such that 100≥n>0 and 10≥k>0.
///
/// Return: The total number of partial permutations P(n,k), modulo 1,000,000.
pub fn rosalind_pper() {
    let contents =
        u64::parse_line(&utils::input_from_file("data/stronghold/rosalind_pper.txt")).unwrap();
    let n = contents[0];
    let k = contents[1];
    println!(
        "{}",
        (utils::ncr(n, k) * utils::factorial(k as usize)) % BigUint::from(1_000_000u64)
    );
}
