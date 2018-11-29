use crate::utils;
use radix::RadixNum;
use std::char;
use std::collections::HashMap;
use std::iter::repeat;

pub fn rosalind_ba1m() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba1m.txt");
    let mut lines = contents.split('\n');
    let (number, k) = (
        lines.next().unwrap().parse::<usize>().unwrap(),
        lines.next().unwrap().parse::<usize>().unwrap(),
    );
    let mut dna = number_to_pattern(number, &"ACGT".chars().collect::<Vec<_>>());
    if dna.len() < k {
        dna = format!(
            "{}{}",
            repeat('A').take(k - dna.len()).collect::<String>(),
            dna
        );
    }
    println!("{}", dna);
}

fn number_to_pattern(number: usize, alphabet: &[char]) -> String {
    let num_alphabets = alphabet.len();
    let alphabet_map: HashMap<char, char> = alphabet
        .iter()
        .enumerate()
        .map(|(n, c)| {
            (
                char::from_digit(n as u32, num_alphabets as u32)
                    .unwrap()
                    .to_ascii_uppercase(),
                *c,
            )
        })
        .collect();
    RadixNum::from(number)
        .with_radix(num_alphabets)
        .unwrap()
        .digits()
        .map(|n| alphabet_map[&n])
        .collect()
}
