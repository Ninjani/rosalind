use std::collections::HashMap;

use failure::Error;

use crate::utility;

/// Catalan Numbers and RNA Secondary Structures
///
/// Given: An RNA string s having the same number of occurrences of 'A' as 'U' and the same number
/// of occurrences of 'C' as 'G'. The length of the string is at most 300 bp.
///
/// Return: The total number of noncrossing perfect matchings of basepair edges in the bonding
/// graph of s, modulo 1,000,000.
pub fn rosalind_cat(filename: &str) -> Result<u64, Error> {
    let rna_string = utility::io::read_fasta_file(filename)?;
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
        let number = count_pairs(&sequence.chars().collect::<Vec<_>>(), &mut cache) % 10u64.pow(6);
        println!("{}", number);
        return Ok(number);
    }
    Ok(0)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cat() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_cat")?;
        assert_eq!(
            rosalind_cat(&input_file)?,
            utility::io::input_from_file(&output_file)?.parse::<u64>()?
        );
        Ok(())
    }
}
