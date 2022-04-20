use std::collections::{HashMap, HashSet};

use anyhow::Error;

use std::path::Path;

/// Locating Restriction Sites
///
/// Given: A DNA string of length at most 1 kbp in FASTA format.
///
/// Return: The position and length of every reverse palindrome in the string having length
/// between 4 and 12. You may return these pairs in any order.
pub fn rosalind_revp(filename: &Path) -> Result<HashSet<(usize, usize)>, Error> {
    let nucleotide_map: HashMap<_, _> = "ATCG".chars().zip("TAGC".chars()).collect();
    let fasta = utility::io::read_fasta_file(filename)?;
    let dna = fasta
        .values()
        .next()
        .ok_or(utility::errors::RosalindOutputError::NoneError)?
        .chars()
        .collect::<Vec<_>>();
    let mut output = HashSet::new();
    for i in 0..dna.len() {
        for length in 4..=12 {
            if i + length > dna.len() {
                continue;
            }
            if dna[i] == nucleotide_map[&dna[i + length - 1]]
                && is_reverse_palindrome(&dna[(i + 1)..(i + length - 1)], &nucleotide_map)
            {
                output.insert((i + 1, length));
            }
        }
    }
    println!(
        "{}",
        output
            .iter()
            .map(|(pos, length)| format!("{} {}", pos, length))
            .collect::<Vec<_>>()
            .join("\n")
    );
    Ok(output)
}

/// Checks if a dna string is a reverse palindrome
fn is_reverse_palindrome(dna: &[char], nucleotide_map: &HashMap<char, char>) -> bool {
    if dna.len() % 2 != 0 {
        return false;
    }
    for i in 0..(dna.len() / 2) {
        if dna[i] != nucleotide_map[&dna[dna.len() - i - 1]] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn revp() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_revp")?;
        let output: HashSet<(usize, usize)> = utility::io::input_from_file(&output_file)?
            .split('\n')
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect();
        assert_eq!(rosalind_revp(&input_file)?, output);
        Ok(())
    }
}
