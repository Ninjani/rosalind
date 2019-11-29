use std::collections::HashMap;

use failure::Error;

use crate::stronghold::r3_revc::reverse_complement;
use crate::textbook_track::r23_ba1b::get_most_frequent_kmers;
use crate::textbook_track::r30_ba1i::get_mismatch_sequences;
use utility;
use utility::io::Parseable;

pub fn rosalind_ba1j() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba1j.txt")?;
    let lines = contents.split('\n').collect::<Vec<_>>();
    let numbers = usize::parse_line(lines[1])?;
    let (k, mismatch) = (numbers[0], numbers[1]);
    let counts_tuple = get_sorted_kmer_counts_approx_revc(lines[0], k, mismatch);
    println!(
        "{}",
        utility::io::format_array(&get_most_frequent_kmers(&counts_tuple))
    );
    Ok(())
}

fn get_sorted_kmer_counts_approx_revc(
    text: &str,
    k: usize,
    mismatch: usize,
) -> Vec<(String, usize)> {
    let text: Vec<_> = text.chars().collect();
    let mut counts = HashMap::new();
    for i in 0..=(text.len() - k) {
        let text_kmer: String = text[i..(i + k)].iter().collect();
        for kmer in get_mismatch_sequences(&text_kmer, mismatch) {
            *counts.entry(reverse_complement(&kmer)).or_insert(0usize) += 1;
            *counts.entry(kmer).or_insert(0usize) += 1;
        }
    }
    let mut counts_tuple = counts.into_iter().collect::<Vec<_>>();
    counts_tuple.sort_by(|a, b| b.1.cmp(&a.1));
    counts_tuple
}
