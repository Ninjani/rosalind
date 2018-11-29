use crate::utils;
use crate::utils::Parseable;
use std::collections::HashMap;

/// Comparing Spectra with the Spectral Convolution
///
/// Given: Two multisets of positive real numbers S1 and S2. The size of each multiset is at most 200.
///
/// Return: The largest multiplicity of S1⊖S2, as well as the absolute value of the number x maximizing (S1⊖S2)(x) (you may return any such value if multiple solutions exist).
pub fn rosalind_conv() {
    let contents = utils::input_from_file("data/stronghold/rosalind_conv.txt");
    let mut lines = contents.split('\n');
    let multiset_a = f64::parse_line(lines.next().unwrap()).unwrap();
    let multiset_b = f64::parse_line(lines.next().unwrap()).unwrap();
    let multiset_a_b = get_minkowski_difference(&multiset_a, &multiset_b);
    let (multiplicity, x) = get_max_multiplicity(&multiset_a_b);
    println!("{}\n{}", multiplicity, x)
}

pub fn get_max_multiplicity(multiset: &[f64]) -> (usize, f64) {
    let mut counts = HashMap::new();
    for s in multiset {
        *counts.entry(format!("{:.5}", s)).or_insert(0usize) += 1;
    }
    let mut counts_tuple = counts.into_iter().collect::<Vec<_>>();
    counts_tuple.sort_by(|a, b| b.1.cmp(&a.1));
    let max_tuple = &counts_tuple[0];
    (max_tuple.1, max_tuple.0.parse::<f64>().unwrap())
}

pub fn get_minkowski_difference(multiset_s1: &[f64], multiset_s2: &[f64]) -> Vec<f64> {
    let mut multiset_s1_s2 = Vec::new();
    for s1 in multiset_s1 {
        for s2 in multiset_s2 {
            multiset_s1_s2.push(*s1 - *s2);
        }
    }
    multiset_s1_s2
}
