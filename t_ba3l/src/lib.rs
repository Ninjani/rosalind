use failure::Error;
use itertools::Itertools;

use crate::textbook_track::r45_ba3b::reverse_kmerize;
use utility;

pub type PairedRead = (String, String);

pub fn rosalind_ba3l() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba3l.txt")?;
    let (paired_reads, k, d) = read_paired_reads(&contents);
    println!(
        "{}",
        get_string_spelled_by_gapped_patterns(&paired_reads.iter().collect::<Vec<_>>(), k, d)
            .unwrap()
    );
    Ok(())
}

pub fn read_paired_reads(contents: &str) -> (Vec<PairedRead>, usize, usize) {
    let mut lines = contents.split('\n');
    let (k, d) = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    (
        lines
            .map(|l| {
                l.trim()
                    .split('|')
                    .map(|s| s.to_owned())
                    .collect_tuple()
                    .unwrap()
            })
            .collect(),
        k,
        d,
    )
}

pub fn get_string_spelled_by_gapped_patterns(
    gapped_patterns: &[&PairedRead],
    k: usize,
    d: usize,
) -> Option<String> {
    let first_patterns = gapped_patterns
        .iter()
        .map(|(k, _)| k.as_str())
        .collect::<Vec<_>>();
    let second_patterns = gapped_patterns
        .iter()
        .map(|(_, k)| k.as_str())
        .collect::<Vec<_>>();
    let mut prefix_string: Vec<_> = reverse_kmerize(&first_patterns).chars().collect();
    let suffix_string: Vec<_> = reverse_kmerize(&second_patterns).chars().collect();
    for i in (k + d + 1)..prefix_string.len() {
        if prefix_string[i] != suffix_string[i - k - d] {
            return None;
        }
    }
    let suffix_length = suffix_string.len();
    prefix_string.extend(suffix_string.into_iter().skip(suffix_length - k - d));
    Some(prefix_string.into_iter().collect())
}
