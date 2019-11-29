use std::char;
use std::collections::HashMap;

use failure::Error;
use radix::RadixNum;

use utility;

/// Enumerating k-mers Lexicographically
///
/// Given: A collection of at most 10 symbols defining an ordered alphabet, and a positive integer n (n≤10).
///
/// Return: All strings of length n that can be formed from the alphabet, ordered lexicographically (use the standard order of symbols in the English alphabet).
pub fn rosalind_lexf(filename: &str) -> Result<Vec<String>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let parts: Vec<_> = input.split('\n').collect();
    let alphabets: Vec<_> = parts[0]
        .split(' ')
        .map(|a| a.chars().next().unwrap())
        .collect();
    let length = parts[1].parse::<usize>()?;
    let output = enumerate_lex(&alphabets, length);
    for line in &output {
        println!("{}", line);
    }
    Ok(output)
}

/// Definitely a hack: uses base-10 conversion to convert between decimal and alphabet length
pub fn enumerate_lex_2(alphabets: Vec<char>, length: usize) -> impl Iterator<Item=String> {
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
    let num_strings = num_alphabets.pow(length as u32);
    (0..num_strings).map(move |i| {
        let i_digits = RadixNum::from(i)
            .with_radix(num_alphabets)
            .unwrap()
            .digits()
            .collect::<Vec<_>>();
        (0..(length - i_digits.len()))
            .map(|_| alphabet_map[&'0'])
            .collect::<String>()
            + &i_digits
            .into_iter()
            .map(|n| alphabet_map[&n])
            .collect::<String>()
    })
}

pub fn enumerate_lex(alphabet: &[char], length: usize) -> Vec<String> {
    if length == 1 {
        alphabet.iter().map(|i| i.to_string()).collect()
    } else {
        let strings = enumerate_lex(alphabet, length - 1);
        alphabet
            .iter()
            .flat_map(|c| strings.iter().map(move |rest| format!("{}{}", c, rest)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexf() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_lexf")?;
        let output = utility::io::input_from_file(&output_file)?;
        let result = rosalind_lexf(&input_file)?;
        assert!(result
            .into_iter()
            .zip(output.split('\n'))
            .all(|(x, y)| x == y));
        Ok(())
    }
}
