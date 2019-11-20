use failure::Error;
use num::BigUint;

use crate::utility;

/// Perfect Matchings and RNA Secondary Structures
///
/// Given: An RNA string s of length at most 80 bp having the same number of occurrences of 'A'
/// as 'U' and the same number of occurrences of 'C' as 'G'.
///
/// Return: The total possible number of perfect matchings of basepair edges in the bonding graph of s.
pub fn rosalind_pmch(filename: &str) -> Result<BigUint, Error> {
    let sequences = utility::io::read_fasta_file(filename)?;
    let (_, sequence) = sequences.iter().collect::<Vec<_>>()[0];
    let nucleotide_counts = utility::string::char_counter(sequence);
    let result = utility::math::factorial(nucleotide_counts[&'A'])
        * utility::math::factorial(nucleotide_counts[&'C']);
    println!("{}", result);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pmch() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_pmch")?;
        let output = utility::io::input_from_file(&output_file)?.parse::<BigUint>()?;
        assert_eq!(rosalind_pmch(&input_file)?, output);
        Ok(())
    }
}
