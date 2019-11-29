use failure::Error;

use utility;

/// Transcribing DNA into RNA
///
/// Given: A DNA string t having length at most 1000 nt.
///
/// Return: The transcribed RNA string of t.
pub fn rosalind_rna(filename: &str) -> Result<String, Error> {
    let input = utility::io::input_from_file(filename)?;
    let output = transcribe(&input);
    println!("{}", output);
    Ok(output)
}

/// Transcribe DNA string into RNA
pub fn transcribe(dna: &str) -> String {
    dna.to_ascii_uppercase().replace("T", "U")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rna() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_rna")?;
        assert_eq!(
            rosalind_rna(&input_file)?,
            utility::io::input_from_file(&output_file)?.trim()
        );
        Ok(())
    }
}
