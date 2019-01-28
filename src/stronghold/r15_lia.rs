use crate::utils;
use crate::utils::Parseable;
use failure::Error;
use num::ToPrimitive;

/// Independent Alleles
///
/// Given: Two positive integers k (k≤7) and N (N≤2k). In this problem, we begin with Tom, who in the 0th generation has genotype Aa Bb. Tom has two children in the 1st generation, each of whom has two children, and so on. Each organism always mates with an organism having genotype Aa Bb.
///
/// Return: The probability that at least N Aa Bb organisms will belong to the k-th generation of Tom's family tree (don't count the Aa Bb mates at each level). Assume that Mendel's second law holds for the factors.
pub fn rosalind_lia() -> Result<(), Error> {
    let contents = u64::parse_line(&utils::input_from_file("data/stronghold/rosalind_lia.txt"))?;
    let k = contents[0];
    let n = contents[1];
    let total = 2u64.pow(k as u32);
    println!(
        "{}",
        (n..=total)
            .map(|i| utils::ncr(total, i).to_f64().unwrap()
                * 0.25f64.powi(i as i32)
                * 0.75f64.powi((total - i) as i32))
            .sum::<f64>()
    );
    Ok(())
}
