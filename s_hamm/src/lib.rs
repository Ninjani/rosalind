use failure::Error;

use utility;
use utility::errors;

/// Counting Point Mutations
///
/// Given: Two DNA strings s and t of equal length (not exceeding 1 kbp).
///
/// Return: The Hamming distance d_H(s,t).
pub fn rosalind_hamm(filename: &str) -> Result<usize, Error> {
    let input = utility::io::input_from_file(filename)?;
    let sequences = input.split('\n').collect::<Vec<&str>>();
    let (sequence_1, sequence_2) = (
        sequences.get(0).ok_or_else(|| {
            errors::RosalindParseError::InputFormatError(String::from("first sequence missing"))
        })?,
        sequences.get(1).ok_or_else(|| {
            errors::RosalindParseError::InputFormatError(String::from("second sequence missing"))
        })?,
    );
    let output = utility::string::hamming(sequence_1, sequence_2);
    println!("{}", output);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamm() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_hamm")?;
        assert_eq!(
            rosalind_hamm(&input_file)?,
            utility::io::input_from_file(&output_file)?.parse::<usize>()?
        );
        Ok(())
    }
}
