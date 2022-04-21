use anyhow::Error;
use std::collections::HashMap;

use std::path::Path;

/// Dictionaries
///
/// Given:  A string s of length at most 10000 letters.
///
/// Return: The number of occurrences of each word in s, where words are separated by spaces.
/// Words are case-sensitive, and the lines in the output can be in any order.
pub fn rosalind_ini6(filename: &Path) -> Result<HashMap<String, usize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut result = HashMap::new();
    for word in input.split_whitespace() {
        let count = result.entry(word.to_owned()).or_insert(0);
        *count += 1;
    }
    for (word, count) in &result {
        println!("{} {}", word, count);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ini6() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_ini6")?;
        let output = utility::io::input_from_file(&output_file)?
            .split('\n')
            .map(|line| {
                let mut parts = line.split_whitespace();
                let word = parts.next().unwrap().to_owned();
                let count = parts.next().unwrap().parse::<usize>().unwrap();
                (word, count)
            })
            .collect::<HashMap<String, usize>>();
        assert_eq!(rosalind_ini6(&input_file)?, output);
        Ok(())
    }
}
