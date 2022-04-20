use std::collections::{HashMap, HashSet};

use anyhow::Error;

use s_prot::translate;
use s_revc::reverse_complement;
use s_rna::transcribe;
use std::path::Path;

/// Open Reading Frames
///
/// Given: A DNA string s of length at most 1 kbp in FASTA format.
///
/// Return: Every distinct candidate protein string that can be translated from ORFs of s.
/// Strings can be returned in any order.
pub fn rosalind_orf(filename: &Path) -> Result<HashSet<String>, Error> {
    let fasta = utility::io::read_fasta_file(filename)?;
    let dna = fasta.values().collect::<Vec<_>>()[0];
    let revc_dna = reverse_complement(dna);
    let (rna, revc_rna) = (transcribe(dna), transcribe(&revc_dna));
    let codons = utility::io::get_codon_to_aa()?;
    let output: HashSet<_> = find_proteins(&rna, &codons)
        .union(&find_proteins(&revc_rna, &codons))
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    println!("{}", output.iter().cloned().collect::<Vec<_>>().join("\n"));
    Ok(output)
}

/// Finds all possible proteins that can be translated from an RNA string
pub fn find_proteins(rna: &str, codons: &HashMap<String, String>) -> HashSet<String> {
    let mut proteins = HashSet::new();
    for i in 0..3 {
        let chunks = utility::string::sub_strings(&rna[i..], 3)
            .into_iter()
            .enumerate()
            .filter(|(_, chunk)| chunk == utility::io::START_CODON)
            .collect::<Vec<_>>();
        for (n, _) in chunks {
            if let Some(protein) = translate(&rna[(n * 3 + i)..], codons) {
                proteins.insert(protein);
            }
        }
    }
    proteins
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orf() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_orf")?;
        let output = utility::io::input_from_file(&output_file)?;
        assert_eq!(
            rosalind_orf(&input_file)?,
            output
                .split('\n')
                .map(|s| s.to_owned())
                .collect::<HashSet<_>>()
        );
        Ok(())
    }
}
