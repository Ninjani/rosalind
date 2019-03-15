use crate::utils;
use radix::RadixNum;
use std::char;
use hashbrown::HashMap;
use failure::Error;

pub fn rosalind_ba1l() -> Result<(), Error> {
    let dna = utils::input_from_file("data/textbook_track/rosalind_ba1l.txt");
    println!(
        "{}",
        pattern_to_number(&dna, &"ACGT".chars().collect::<Vec<_>>())?
    );
    Ok(())
}

fn pattern_to_number(pattern: &str, alphabet: &[char]) -> Result<usize, Error> {
    let num_alphabets = alphabet.len();
    let alphabet_map: HashMap<char, char> = alphabet
        .iter()
        .enumerate()
        .map(|(n, c)| {
            (
                *c,
                char::from_digit(n as u32, num_alphabets as u32)
                    .unwrap()
                    .to_ascii_uppercase(),
            )
        })
        .collect();
    Ok(RadixNum::from_str(
        &pattern
            .chars()
            .map(|c| alphabet_map[&c])
            .collect::<String>(),
        num_alphabets,
    )?
    .as_decimal()?)
}
