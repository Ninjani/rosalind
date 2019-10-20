use std::collections::HashMap;

use failure::Error;

use crate::utility;

/// Translating RNA into Protein
///
/// Given: An RNA string s corresponding to a strand of mRNA (of length at most 10 kbp).
///
/// Return: The protein string encoded by s.
pub fn rosalind_prot(filename: &str) -> Result<String, Error> {
    let input = utility::io::input_from_file(filename)?;
    let codons = utility::io::get_codon_to_aa()?;
    Ok(
        translate(&input, &codons)
            .ok_or_else(|| utility::errors::RosalindOutputError::NoneError)?,
    )
}

/// Get protein from RNA string (decodes till Stop codon reached)
pub fn translate(rna: &str, codons: &HashMap<String, String>) -> Option<String> {
    let mut protein = String::with_capacity(rna.len() / 3);
    for chunk in utility::string::sub_strings(rna, 3) {
        match codons.get(&chunk) {
            Some(amino_acid) => {
                if amino_acid == utility::io::STOP_CODON_AA {
                    return Some(protein);
                } else {
                    protein.push_str(amino_acid);
                }
            }
            None => return None,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prot() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_prot")?;
        assert_eq!(
            rosalind_prot(&input_file)?,
            utility::io::input_from_file(&output_file)?.trim()
        );
        Ok(())
    }
}
