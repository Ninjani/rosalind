use anyhow::Error;

use std::path::Path;

/// Finding a motif in DNA
///
/// Given: Two DNA strings s and t (each of length at most 1 kbp).
///
/// Return: All locations of t as a substring of s.
pub fn rosalind_subs(filename: &Path) -> Result<Vec<usize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let dna_motif = input.split('\n').collect::<Vec<&str>>();
    let dna = dna_motif.get(0).ok_or_else(|| {
        utility::errors::RosalindParseError::InputFormatError(String::from("Missing dna"))
    })?;
    let motif = dna_motif.get(1).ok_or_else(|| {
        utility::errors::RosalindParseError::InputFormatError(String::from("Missing motif"))
    })?;
    let output = utility::string::find_motifs(motif, dna)
        .into_iter()
        .map(|x| x + 1)
        .collect::<Vec<usize>>();
    println!("{}", utility::io::format_array(&output));
    Ok(output)
}

#[cfg(test)]
mod tests {
    use utility::io::Parseable;

    use super::*;

    #[test]
    fn subs() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_subs")?;
        let mut output = usize::parse_line(&utility::io::input_from_file(&output_file)?)?;
        output.sort();
        assert_eq!(rosalind_subs(&input_file)?, output);
        Ok(())
    }
}
