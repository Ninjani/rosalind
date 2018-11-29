use crate::utils;
use std::collections::HashMap;

/// Catalan Numbers and RNA Secondary Structures
///
/// Given: An RNA string s having the same number of occurrences of 'A' as 'U' and the same number of occurrences of 'C' as 'G'. The length of the string is at most 300 bp.
///
/// Return: The total number of noncrossing perfect matchings of basepair edges in the bonding graph of s, modulo 1,000,000.
pub fn rosalind_cat() {
    let rna_string = utils::read_fasta_file("data/stronghold/rosalind_cat.txt");
    for (_, sequence) in rna_string {
        let mut cache: HashMap<_, _> = vec![
            ("".into(), 1),
            ("CG".into(), 1),
            ("AU".into(), 1),
            ("GC".into(), 1),
            ("UA".into(), 1),
        ]
        .into_iter()
        .collect();
        println!(
            "{}",
            count_pairs(&sequence.chars().collect::<Vec<_>>(), &mut cache) % 10u64.pow(6)
        );
    }
}

fn count_pairs(sequence: &[char], cache: &mut HashMap<String, u64>) -> u64 {
    if !cache.contains_key(&sequence.iter().collect::<String>()) {
        let mut num_pairs = 0;
        for i in (1..sequence.len()).step_by(2) {
            num_pairs += count_pairs(&sequence.to_vec()[1..i], cache)
                * cache
                    .get(&format!("{}{}", sequence[0], sequence[i]))
                    .unwrap_or(&0)
                * count_pairs(&sequence.to_vec()[(i + 1)..], cache);
        }
        cache.insert(sequence.iter().collect(), num_pairs % 10u64.pow(6));
    }
    cache[&sequence.iter().collect::<String>()]
}

fn catalan_number(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        (1..=n)
            .map(|k| catalan_number(k - 1) * catalan_number(n - k))
            .sum()
    }
}
