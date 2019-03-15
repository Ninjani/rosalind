use crate::stronghold::r23_lexf::enumerate_lex_2;
use crate::utils;
use hashbrown::HashMap;
use std::iter::repeat;
use failure::Error;

pub fn rosalind_ba1k() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba1k.txt");
    let lines = contents.split('\n').collect::<Vec<_>>();
    let (text, k) = (
        lines[0],
        lines[1].parse::<usize>()?,
    );
    utils::print_array(&get_frequency_array(&text, k));
    Ok(())
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
