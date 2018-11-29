use crate::stronghold::r2_rna::transcribe;
use crate::stronghold::r3_revc::reverse_complement;
use crate::stronghold::r8_prot::translate;
use crate::utils;
use std::collections::HashSet;

/// Finds all possible proteins that can be translated from an RNA string
pub fn find_proteins(rna: &str) -> HashSet<String> {
    let mut proteins = HashSet::new();
    for i in 0..3 {
        let chunks = utils::sub_strings(&rna[i..], 3)
            .into_iter()
            .enumerate()
            .filter(|(_, chunk)| chunk == utils::START_CODON)
            .collect::<Vec<_>>();
        for (n, _) in chunks {
            if let Some(protein) = translate(&rna[(n * 3 + i)..]) {
                proteins.insert(protein);
            }
        }
    }
    proteins
}

/// Open Reading Frames
///
/// Given: A DNA string s of length at most 1 kbp in FASTA format.
///
/// Return: Every distinct candidate protein string that can be translated from ORFs of s. Strings can be returned in any order.
pub fn rosalind_orf() {
    let fasta = utils::read_fasta_file("data/stronghold/rosalind_orf.txt");
    let dna = fasta.values().next().unwrap();
    let revc_dna = reverse_complement(dna);
    let (rna, revc_rna) = (transcribe(&dna), transcribe(&revc_dna));
    for protein in find_proteins(&rna).union(&find_proteins(&revc_rna)) {
        println!("{}", protein);
    }
}
