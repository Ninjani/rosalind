use crate::utils;
use crate::utils::Parseable;
use failure::Error;
use num::BigUint;

/// Introduction to Alternative Splicing
///
/// Given: Positive integers n and m with 0≤m≤n≤2000.
///
/// Return: The sum of combinations C(n,k) for all k satisfying m≤k≤n, modulo 1,000,000.
pub fn rosalind_aspc() -> Result<(), Error> {
    let nm = u64::parse_line(&utils::input_from_file("data/stronghold/rosalind_aspc.txt"))?;
    let (n, m) = (nm[0], nm[1]);
    println!(
        "{}",
        (m..=n)
            .map(|k| utils::ncr(n, k) % BigUint::from(10u64.pow(6)))
            .sum::<BigUint>()
            % BigUint::from(10u64.pow(6))
    );
    Ok(())
}
