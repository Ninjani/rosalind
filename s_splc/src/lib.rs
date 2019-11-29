use failure::Error;

use crate::stronghold::r2_rna::transcribe;
use crate::stronghold::r8_prot::translate;
use utility;

/// RNA Splicing
///
/// Given: A DNA string s (of length at most 1 kbp) and a collection of substrings of s acting
/// as introns. All strings are given in FASTA format.
///
/// Return: A protein string resulting from transcribing and translating the exons of s.
/// (Note: Only one solution will exist for the dataset provided.)
pub fn rosalind_splc(filename: &str) -> Result<String, Error> {
    let sequences = utility::io::read_fasta_file(filename)?;
    let dna_key = sequences
        .keys()
        .map(|key| (key, sequences[key].len()))
        .max_by(|a, b| a.1.cmp(&b.1))
        .ok_or_else(|| utility::errors::RosalindOutputError::NoneError)?
        .0;
    let intron_keys = sequences
        .keys()
        .filter(|key| key != &dna_key)
        .collect::<Vec<_>>();
    let mut exons = sequences[dna_key].chars().collect::<Vec<char>>();
    for intron_key in intron_keys {
        let intron = &sequences[intron_key];
        let remove_indices =
            utility::string::find_motifs(intron, &exons.iter().collect::<String>())
                .into_iter()
                .flat_map(|index| index..(index + intron.len()))
                .collect::<Vec<_>>();
        exons = exons
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !remove_indices.contains(i))
            .map(|(_, c)| c)
            .collect::<Vec<_>>();
    }
    let codons = utility::io::get_codon_to_aa()?;
    Ok(
        translate(&transcribe(&exons.iter().collect::<String>()), &codons)
            .ok_or_else(|| utility::errors::RosalindOutputError::NoneError)?,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splc() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_splc")?;
        let output = utility::io::input_from_file(&output_file)?;
        assert_eq!(rosalind_splc(&input_file)?, output.trim());
        Ok(())
    }
}
