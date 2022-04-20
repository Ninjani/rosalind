use anyhow::Error;

use std::path::Path;

/// Inferring mRNA from Protein
///
/// Given: A protein string of length at most 1000 aa.
///
/// Return: The total number of different RNA strings from which the
/// protein could have been translated, modulo 1,000,000. (Don't neglect the
/// importance of the stop codon in protein translation.)
pub fn rosalind_mrna(filename: &Path) -> Result<usize, Error> {
    let input = utility::io::input_from_file(filename)?;
    let aa_to_codon = utility::io::get_aa_to_codon()?;
    let mut count = 1usize;
    let mod_value = 1_000_000usize;
    for aa in input.chars() {
        count = (count % mod_value) * (aa_to_codon[&aa.to_string()].len() % mod_value)
    }
    let output = (count % mod_value) * aa_to_codon[utility::io::STOP_CODON_AA].len();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mrna() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_mrna")?;
        assert_eq!(
            rosalind_mrna(&input_file)?,
            utility::io::input_from_file(&output_file)?.parse::<usize>()?
        );
        Ok(())
    }
}
