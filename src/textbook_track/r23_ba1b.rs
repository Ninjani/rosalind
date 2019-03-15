use crate::utils;
use hashbrown::HashMap;
use failure::Error;

pub fn rosalind_ba1b() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba1b.txt");
    let lines = contents.split('\n').collect::<Vec<_>>();
    let (text, k) = (
        lines[0],
        lines[1].parse::<usize>()?,
    );
    let counts_tuple = get_sorted_kmer_counts(text, k);
    utils::print_array(&get_most_frequent_kmers(&counts_tuple));
    Ok(())
}

pub fn get_sorted_kmer_counts(text: &str, k: usize) -> Vec<(String, usize)> {
    let text: Vec<_> = text.chars().collect();
    let mut counts = HashMap::new();
    for i in 0..=(text.len() - k) {
        *counts
            .entry(text[i..(i + k)].iter().collect::<String>())
            .or_insert(0usize) += 1;
    }
    let mut counts_tuple = counts.into_iter().collect::<Vec<_>>();
    counts_tuple.sort_by(|a, b| b.1.cmp(&a.1));
    counts_tuple
}

pub fn get_most_frequent_kmers(counts_tuple: &[(String, usize)]) -> Vec<String> {
    let max_count = counts_tuple[0].1;
    let mut max_kmers = Vec::new();
    for tuple in counts_tuple {
        if tuple.1 != max_count {
            break;
        }
        max_kmers.push(tuple.0.clone())
    }
    max_kmers
}
