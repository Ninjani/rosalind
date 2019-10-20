use std::collections::HashMap;

use failure::Error;

use crate::utility;

/// Complementing a Strand of DNA
///
/// Given: A DNA string s of length at most 1000 bp.
///
/// Return: The reverse complement s^c of s
pub fn rosalind_revc(filename: &str) -> Result<String, Error> {
    let input = utility::io::input_from_file(filename)?;
    let output = reverse_complement(&input);
    println!("{}", output);
    Ok(output)
}

/// Get the reverse complement of a DNA string
pub fn reverse_complement(dna: &str) -> String {
    let nucleotide_map: HashMap<_, _> = "ATCG".chars().zip("TAGC".chars()).collect();
    dna.to_ascii_uppercase()
        .chars()
        .rev()
        .map(|c| &nucleotide_map[&c])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revc() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_revc")?;
        assert_eq!(
            rosalind_revc(&input_file)?,
            utility::io::input_from_file(&output_file)?.trim()
        );
        Ok(())
    }
}
