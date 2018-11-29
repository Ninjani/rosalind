use crate::utils;

/// Transcribe DNA string into RNA
pub fn transcribe(dna: &str) -> String {
    dna.to_ascii_uppercase().replace("T", "U")
}

/// Transcribing DNA into RNA
///
/// Given: A DNA string t having length at most 1000 nt.
///
/// Return: The transcribed RNA string of t.
pub fn rosalind_rna() {
    let dna = utils::input_from_file("data/stronghold/rosalind_rna.txt");
    println!("{}", transcribe(&dna));
}
