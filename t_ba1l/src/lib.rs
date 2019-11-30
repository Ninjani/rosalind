use std::char;
use std::collections::HashMap;

use failure::Error;
use radix::RadixNum;

use utility;

pub fn rosalind_ba1l(filename: &str) -> Result<(), Error> {
    let dna = utility::io::input_from_file(filename)?;
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
