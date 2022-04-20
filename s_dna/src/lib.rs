use anyhow::Error;

use std::path::Path;

/// Counting DNA Nucleotides
///
/// Given: A DNA string s of length at most 1000 nt.
///
/// Return: Four integers (separated by spaces) counting the respective number of times
/// that the symbols 'A', 'C', 'G', and 'T' occur in s.
pub fn rosalind_dna(filename: &Path) -> Result<Vec<usize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let counter = utility::string::char_counter(&input);
    let counts = "ACGT"
        .chars()
        .map(|c| *counter.get(&c).unwrap_or(&0usize))
        .collect::<Vec<usize>>();
    println!("{}", utility::io::format_array(&counts));
    Ok(counts)
}

#[cfg(test)]
mod tests {
    use utility::io::Parseable;

    use super::*;

    #[test]
    fn dna() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_dna")?;
        let output = usize::parse_line(&utility::io::input_from_file(&output_file)?)?;
        assert_eq!(rosalind_dna(&input_file)?, output);
        Ok(())
    }
}
