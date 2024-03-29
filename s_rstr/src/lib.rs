use anyhow::Error;
use itertools::Itertools;

use s_prob::nucleotide_probs_from_gc_content;
use utility::io::Parseable;

/// Matching Random Motifs
///
/// Given: A positive integer N≤100000, a number x between 0 and 1, and a DNA string s of length at most 10 bp.
///
/// Return: The probability that if N random DNA strings having the same length as s are constructed with GC-content x
/// (see “Introduction to Random Strings”), then at least one of the strings equals s.
/// We allow for the same random string to be created more than once.
pub fn rosalind_rstr(input: &str) -> Result<f64, Error> {
    let lines: Vec<_> = input.split('\n').collect();
    let (num, gc_content) = f64::parse_line(lines[0])?
        .into_iter()
        .collect_tuple()
        .ok_or(utility::errors::RosalindOutputError::NoneError)?;
    let sequence = lines[1];
    let nucleotide_probs = nucleotide_probs_from_gc_content(gc_content);
    let a_complement = 1.
        - sequence
            .chars()
            .map(|c| nucleotide_probs[&c])
            .product::<f64>();
    Ok(1. - a_complement.powf(num))
}
