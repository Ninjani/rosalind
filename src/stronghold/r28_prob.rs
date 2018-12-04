use crate::utils;
use crate::utils::Parseable;
use std::collections::HashMap;
use failure::Error;

/// Get expected probabilities of each nucleotide from the GC content
pub fn nucleotide_probs_from_gc_content(gc_content: f64) -> HashMap<char, f64> {
    let gc = gc_content / 2.;
    let at = (1. - gc_content) / 2.;
    "ACGT".chars().zip(vec![at, gc, gc, at]).collect()
}

/// Introduction to Random Strings
///
/// Given: A DNA string s of length at most 100 bp and an array A containing at most 20 numbers between 0 and 1.
///
/// Return: An array B having the same length as A in which B[k] represents the common logarithm of the probability that a random string constructed with the GC-content found in A[k] will match s exactly.
pub fn rosalind_prob() -> Result<(), Error> {
    let contents = utils::input_from_file("data/stronghold/rosalind_prob.txt");
    let parts: Vec<_> = contents.split('\n').collect();
    let sequence = parts[0];
    let gc_contents = f64::parse_line(parts[1])?;
    let mut probabilities = Vec::new();
    for gc_content in gc_contents {
        let nucleotide_probs = nucleotide_probs_from_gc_content(gc_content);
        probabilities.push(
            sequence
                .chars()
                .map(|c| nucleotide_probs[&c].log10())
                .sum::<f64>(),
        );
    }
    utils::print_array(&probabilities);
    Ok(())
}
