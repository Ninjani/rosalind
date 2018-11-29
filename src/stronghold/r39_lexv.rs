use crate::utils;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::repeat;

fn cartesian_product_repeat(alphabet: &[char], length: usize) -> Vec<String> {
    (1..=length)
        .flat_map(|l| repeat(alphabet.iter()).take(l).multi_cartesian_product())
        .map(|x| x.into_iter().collect())
        .collect()
}

/// Ordering Strings of Varying Length Lexicographically
///
/// Given: A permutation of at most 12 symbols defining an ordered alphabet 𝒜 and a positive integer n (n≤4).
///
/// Return: All strings of length at most n formed from 𝒜, ordered lexicographically.
/// (Note: As in “Enumerating k-mers Lexicographically”, alphabet order is based on the order in which the symbols are given.)
pub fn rosalind_lexv() {
    let contents = utils::input_from_file("data/stronghold/rosalind_lexv.txt");
    let mut parts = contents.split('\n');
    let alphabet = parts
        .next()
        .unwrap()
        .split(' ')
        .map(|a| a.chars().next().unwrap())
        .collect::<Vec<_>>();
    let alphabet_indices: HashMap<_, _> =
        alphabet.iter().enumerate().map(|(i, c)| (c, i)).collect();
    let length = parts.next().unwrap().parse::<usize>().unwrap();
    let mut strings = cartesian_product_repeat(&alphabet, length);
    strings.sort_by_key(|k| k.chars().map(|c| alphabet_indices[&c]).collect::<Vec<_>>());
    for string in strings {
        println!("{}", string);
    }
}
