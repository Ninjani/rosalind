use crate::utils;

/// Inferring mRNA from Protein
///
/// Given: A protein string of length at most 1000 aa.
///
/// Return: The total number of different RNA strings from which the protein could have been translated, modulo 1,000,000. (Don't neglect the importance of the stop codon in protein translation.)
pub fn rosalind_mrna() {
    let aa_to_codon = utils::get_aa_to_codon();
    let protein = utils::input_from_file("data/stronghold/rosalind_mrna.txt");
    let mut count = 1usize;
    let mod_value = 1_000_000usize;
    for aa in protein.chars() {
        count = (count % mod_value) * (aa_to_codon[&aa.to_string()].len() % mod_value)
    }
    count = (count % mod_value) * aa_to_codon[utils::STOP_CODON_AA].len();
    println!("{}", count);
}
