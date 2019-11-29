use std::char;
use std::collections::HashMap;
use std::iter::repeat;

use failure::Error;
use radix::RadixNum;

use utility;

pub fn rosalind_ba1m() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba1m.txt")?;
    let lines = contents.split('\n').collect::<Vec<_>>();
    let (number, k) = (lines[0].parse::<usize>()?, lines[1].parse::<usize>()?);
    let mut dna = number_to_pattern(number, &"ACGT".chars().collect::<Vec<_>>())?;
    if dna.len() < k {
        dna = format!(
            "{}{}",
            repeat('A').take(k - dna.len()).collect::<String>(),
            dna
        );
    }
    println!("{}", dna);
    Ok(())
}

fn number_to_pattern(number: usize, alphabet: &[char]) -> Result<String, Error> {
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
    Ok(RadixNum::from(number)
        .with_radix(num_alphabets)?
        .digits()
        .map(|n| alphabet_map[&n])
        .collect())
}
