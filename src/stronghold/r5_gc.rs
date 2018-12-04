use crate::utils;
use failure::{err_msg, Error};
/// Get frequency of Gs and Cs in a DNA string
fn get_gc_content(sequence: &str) -> f32 {
    sequence.chars().filter(|c| *c == 'C' || *c == 'G').count() as f32 / sequence.len() as f32
}

/// Computing GC Content
///
/// Given: At most 10 DNA strings in FASTA format (of length at most 1 kbp each).
///
/// Return: The ID of the string having the highest GC-content, followed by the GC-content of that string. Rosalind allows for a default error of 0.001 in all decimal answers unless otherwise stated; please see the note on absolute error below.
pub fn rosalind_gc() -> Result<(), Error> {
    let sequences = utils::read_fasta_file("data/stronghold/rosalind_gc.txt");
    let gc_contents = sequences
        .iter()
        .map(|(key, dna)| (key, get_gc_content(dna)));
    let (max_key, max_value) = gc_contents
        .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap()).ok_or(err_msg("NoneError"))?;
    println!("{}\n{}", max_key, max_value * 100.);
    Ok(())
}
