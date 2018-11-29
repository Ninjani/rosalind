use crate::stronghold::r23_lexf::enumerate_lex_2;
use crate::utils;
use std::collections::HashMap;
use std::iter::repeat;

pub fn rosalind_ba1k() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba1k.txt");
    let mut lines = contents.split('\n');
    let (text, k) = (
        lines.next().unwrap(),
        lines.next().unwrap().parse::<usize>().unwrap(),
    );
    utils::print_array(&get_frequency_array(&text, k));
}

fn get_frequency_array(text: &str, k: usize) -> Vec<usize> {
    let kmers = enumerate_lex_2(&"ACGT".chars().collect::<Vec<_>>(), k);
    let mut counts: Vec<_> = repeat(0).take(kmers.len()).collect();
    let kmer_indices: HashMap<_, _> = kmers
        .into_iter()
        .enumerate()
        .map(|(i, kmer)| (kmer, i))
        .collect();
    let text: Vec<_> = text.chars().collect();
    for i in 0..=(text.len() - k) {
        let text_kmer: String = text[i..(i + k)].iter().collect();
        counts[kmer_indices[&text_kmer]] += 1;
    }
    counts
}
