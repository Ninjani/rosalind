use crate::utils;
use failure::Error;
use radix::RadixNum;
use std::char;
use hashbrown::HashMap;
use std::iter::repeat;

/// Definitely a hack: uses base-10 conversion to convert between decimal and alphabet length
pub fn enumerate_lex(alphabets: Vec<char>, length: u32) -> impl Iterator<Item = String> {
    let num_alphabets = alphabets.len();
    let alphabet_map: HashMap<char, char> = alphabets
        .into_iter()
        .enumerate()
        .map(|(n, c)| {
            (
                char::from_digit(n as u32, num_alphabets as u32)
                    .unwrap()
                    .to_ascii_uppercase(),
                c,
            )
        })
        .collect();
    let num_strings = num_alphabets.pow(length);
    (0..num_strings).map(move |i| {
        let i_digits = RadixNum::from(i)
            .with_radix(num_alphabets)
            .unwrap()
            .digits()
            .collect::<Vec<_>>();
        repeat(alphabet_map[&'0'])
            .take(length as usize - i_digits.len())
            .collect::<String>()
            + &i_digits
                .into_iter()
                .map(|n| alphabet_map[&n])
                .collect::<String>()
    })
}

pub fn enumerate_lex_2(alphabet: &[char], length: usize) -> Vec<String> {
    if length == 1 {
        alphabet.iter().map(|i| i.to_string()).collect()
    } else {
        let strings = enumerate_lex_2(alphabet, length - 1);
        alphabet
            .iter()
            .flat_map(|c| strings.iter().map(move |rest| format!("{}{}", c, rest)))
            .collect()
    }
}

/// Enumerating k-mers Lexicographically
///
/// Given: A collection of at most 10 symbols defining an ordered alphabet, and a positive integer n (n≤10).
///
/// Return: All strings of length n that can be formed from the alphabet, ordered lexicographically (use the standard order of symbols in the English alphabet).
pub fn rosalind_lexf() -> Result<(), Error> {
    let contents = utils::input_from_file("data/stronghold/rosalind_lexf.txt");
    let parts: Vec<_> = contents.split('\n').collect();
    let alphabets: Vec<_> = parts[0]
        .split(' ')
        .map(|a| a.chars().next().unwrap())
        .collect();
    let length = parts[1].parse::<u32>()?;
    for i_radix_string in enumerate_lex(alphabets, length) {
        println!("{}", i_radix_string);
    }
    Ok(())
}
