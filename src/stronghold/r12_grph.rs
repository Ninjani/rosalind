use crate::utils;
use std::collections::HashMap;

/// Make graph connecting sequences overlapping by overlap_length
pub fn get_overlap_graph<S: ::std::hash::BuildHasher>(
    sequences: &HashMap<String, String, S>,
    overlap_length: usize,
) -> Vec<(String, String)> {
    let nodes = sequences
        .iter()
        .map(|(key, sequence)| {
            let length = sequence.len();
            let prefix = &sequence[0..overlap_length];
            let suffix = &sequence[(length - overlap_length)..];
            (key, prefix, suffix)
        })
        .collect::<Vec<(&String, &str, &str)>>();
    let mut edges = Vec::new();
    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if i != j {
                let (key_0, _, suffix_0) = nodes[i];
                let (key_1, prefix_1, _) = nodes[j];
                if suffix_0 == prefix_1 {
                    edges.push((key_0.to_owned(), key_1.to_owned()));
                }
            }
        }
    }
    edges
}

/// Overlap Graphs
///
/// Given: A collection of DNA strings in FASTA format having total length at most 10 kbp.
///
/// Return: The adjacency list corresponding to O_3. You may return edges in any order.
pub fn rosalind_grph() {
    let sequences = utils::read_fasta_file("data/stronghold/rosalind_grph.txt");
    let overlap_length = 3;
    for (key_0, key_1) in get_overlap_graph(&sequences, overlap_length) {
        println!("{} {}", key_0, key_1);
    }
}
