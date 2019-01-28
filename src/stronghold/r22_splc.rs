use crate::stronghold::r2_rna::transcribe;
use crate::stronghold::r8_prot::translate;
use crate::utils;
use failure::{err_msg, Error};

/// RNA Splicing
///
/// Given: A DNA string s (of length at most 1 kbp) and a collection of substrings of s acting as introns. All strings are given in FASTA format.
///
/// Return: A protein string resulting from transcribing and translating the exons of s. (Note: Only one solution will exist for the dataset provided.)
pub fn rosalind_splc() -> Result<(), Error> {
    let sequences = utils::read_fasta_file("data/stronghold/rosalind_splc.txt");
    let dna_key = sequences
        .keys()
        .map(|key| (key, sequences[key].len()))
        .max_by(|a, b| a.1.cmp(&b.1))
        .ok_or_else(|| err_msg("NoneError"))?
        .0;
    let intron_keys = sequences
        .keys()
        .filter(|key| key != &dna_key)
        .collect::<Vec<_>>();
    let mut exons = sequences[dna_key].chars().collect::<Vec<char>>();
    for intron_key in intron_keys {
        let intron = &sequences[intron_key];
        let remove_indices = utils::find_motifs(intron, &exons.iter().collect::<String>())
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
    println!(
        "{}",
        translate(&transcribe(&exons.iter().collect::<String>()))
            .ok_or_else(|| err_msg("NoneError"))?
    );
    Ok(())
}
