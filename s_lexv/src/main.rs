use std::collections::HashMap;

use failure::Error;
use itertools::Itertools;

use crate::utility;

/// Ordering Strings of Varying Length Lexicographically
///
/// Given: A permutation of at most 12 symbols defining an ordered alphabet 𝒜 and a
/// positive integer n (n≤4).
///
/// Return: All strings of length at most n formed from 𝒜, ordered lexicographically.
/// (Note: As in “Enumerating k-mers Lexicographically”, alphabet order is based on the order
/// in which the symbols are given.)
pub fn rosalind_lexv(filename: &str) -> Result<Vec<String>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let parts: Vec<_> = input.split('\n').collect();
    let alphabet = parts[0]
        .split(' ')
        .map(|a| a.chars().next().unwrap())
        .collect::<Vec<_>>();
    let alphabet_indices: HashMap<_, _> =
        alphabet.iter().enumerate().map(|(i, c)| (c, i)).collect();
    let length = parts[1].parse::<usize>()?;
    let mut strings = cartesian_product_repeat(&alphabet, length);
    strings.sort_by_key(|k| k.chars().map(|c| alphabet_indices[&c]).collect::<Vec<_>>());
    println!("{}", strings.join("\n"));
    Ok(strings)
}

fn cartesian_product_repeat(alphabet: &[char], length: usize) -> Vec<String> {
    (1..=length)
        .flat_map(|l| (0..l).map(|_| alphabet.iter()).multi_cartesian_product())
        .map(|x| x.into_iter().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexv() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_lexv")?;
        let result = rosalind_lexv(&input_file)?;
        assert!(utility::io::input_from_file(&output_file)?
            .split('\n')
            .zip(result.into_iter())
            .all(|(x, y)| x == y));
        Ok(())
    }
}
