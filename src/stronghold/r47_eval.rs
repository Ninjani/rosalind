use crate::stronghold::r28_prob::nucleotide_probs_from_gc_content;
use crate::utils;
use crate::utils::Parseable;

/// Expected Number of Restriction Sites
///
/// Given: A positive integer n (n≤1,000,000), a DNA string s of even length at most 10, and an array A of length at most 20, containing numbers between 0 and 1.
///
/// Return: An array B having the same length as A in which B[i] represents the expected number of times that s will appear as a substring of a random DNA string t of length n, where t is formed with GC-content A[i].
pub fn rosalind_eval() {
    let contents = utils::input_from_file("data/stronghold/rosalind_eval.txt");
    let mut lines = contents.split('\n');
    let length = lines.next().unwrap().parse::<usize>().unwrap();
    let substring = lines.next().unwrap();
    let gc_contents = f64::parse_line(lines.next().unwrap()).unwrap();
    let mut b = Vec::with_capacity(gc_contents.len());
    for gc_content in gc_contents {
        let nucleotide_probs = nucleotide_probs_from_gc_content(gc_content);
        let probability_substring = substring
            .chars()
            .map(|c| nucleotide_probs[&c])
            .product::<f64>();
        b.push(probability_substring * ((length - substring.len() + 1) as f64));
    }
    utils::print_array(&b);
}
