use crate::utils;
use failure::{err_msg, Error};
use hashbrown::HashMap;

/// Checks if a dna string is a reverse palindrome
fn check_palindrome(dna: &[char], nucleotide_map: &HashMap<char, char>) -> bool {
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

/// Locating Restriction Sites
///
/// Given: A DNA string of length at most 1 kbp in FASTA format.
///
/// Return: The position and length of every reverse palindrome in the string having length between 4 and 12. You may return these pairs in any order.
pub fn rosalind_revp() -> Result<(), Error> {
    let nucleotide_map: HashMap<_, _> = "ATCG".chars().zip("TAGC".chars()).collect();
    let fasta = utils::read_fasta_file("data/stronghold/rosalind_revp.txt");
    let dna = fasta
        .values()
        .next()
        .ok_or_else(|| err_msg("NoneError"))?
        .chars()
        .collect::<Vec<_>>();
    for i in 0..dna.len() {
        for length in 4..=12 {
            if i + length > dna.len() {
                continue;
            }
            if dna[i] == nucleotide_map[&dna[i + length - 1]]
                && check_palindrome(&dna[(i + 1)..(i + length - 1)], &nucleotide_map)
            {
                println!("{} {}", i + 1, length);
            }
        }
    }
    Ok(())
}
