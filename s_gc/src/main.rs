use failure::{err_msg, Error};

use crate::utility;

/// Computing GC Content
///
/// Given: At most 10 DNA strings in FASTA format (of length at most 1 kbp each).
///
/// Return: The ID of the string having the highest GC-content,
/// followed by the GC-content of that string.
/// Rosalind allows for a default error of 0.001 in all decimal answers unless otherwise stated;
/// please see the note on absolute error below.
pub fn rosalind_gc(filename: &str) -> Result<(String, f32), Error> {
    let sequences = utility::io::read_fasta_file(filename)?;
    let gc_contents = sequences
        .iter()
        .map(|(key, dna)| (key, get_gc_content(dna)));
    let (max_key, max_value) = gc_contents
        .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
        .ok_or_else(|| err_msg("NoneError"))?;
    println!("{}\n{}", max_key, max_value * 100.);
    Ok((max_key.to_owned(), max_value * 100.))
}

/// Get frequency of Gs and Cs in a DNA string
fn get_gc_content(sequence: &str) -> f32 {
    sequence.chars().filter(|c| *c == 'C' || *c == 'G').count() as f32 / sequence.len() as f32
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use itertools::Itertools;

    use super::*;

    #[test]
    fn gc() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_gc")?;
        let (key, value) = rosalind_gc(&input_file)?;
        let output = utility::io::input_from_file(&output_file)?;
        let (output_key, output_value) = output.trim().split_whitespace().collect_tuple().unwrap();
        let output_value = output_value.parse::<f32>()?;
        assert_eq!(key, output_key);
        assert_approx_eq!(
            value,
            output_value,
            utility::testing::ROSALIND_FLOAT_ERROR_F32
        );
        Ok(())
    }
}
