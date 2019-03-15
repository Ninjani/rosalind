use crate::stronghold::r23_lexf::enumerate_lex;
use crate::utils;
use hashbrown::HashMap;
use std::iter::repeat;

/// k-Mer Composition
///
/// Given: A DNA string s in FASTA format (having length at most 100 kbp).
///
/// Return: The 4-mer composition of s.
pub fn rosalind_kmer() {
    let alphabets = vec!['A', 'C', 'G', 'T'];
    let dna = utils::read_fasta_file("data/stronghold/rosalind_kmer.txt");
    let kmer_indices: HashMap<String, usize> = enumerate_lex(alphabets, 4)
        .enumerate()
        .map(|(i, k)| (k, i))
        .collect();
    for (_, sequence) in dna {
        let mut counts: Vec<usize> = repeat(0).take(kmer_indices.len()).collect();
        for kmer in utils::kmerize(&sequence, 4) {
            counts[kmer_indices[&kmer]] += 1;
        }
        utils::print_array(&counts);
    }
}
