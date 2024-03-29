use std::collections::HashMap;

use anyhow::Error;

use s_lexf::enumerate_lex;
use std::path::Path;

pub fn rosalind_ba1k(filename: &Path) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let lines = contents.split('\n').collect::<Vec<_>>();
    let (text, k) = (lines[0], lines[1].parse::<usize>()?);
    println!(
        "{}",
        utility::io::format_array(&get_frequency_array(text, k))
    );
    Ok(())
}

fn get_frequency_array(text: &str, k: usize) -> Vec<usize> {
    let kmers = enumerate_lex(&"ACGT".chars().collect::<Vec<_>>(), k);
    let mut counts: Vec<_> = (0..kmers.len()).map(|_| 0).collect();
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
