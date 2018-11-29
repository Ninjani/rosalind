use crate::textbook_track::r59_ba4c::get_aa_to_mass_usize;
use crate::textbook_track::r66_ba4j::get_prefix_masses;
use crate::utils;
use std::collections::HashMap;

/// Convert a Peptide into a Peptide Vector
///
/// Given: A peptide P.
///
/// Return: The peptide vector of P.
pub fn rosalind_ba11c() {
    let peptide = utils::input_from_file("data/textbook_track/rosalind_ba11c.txt");
    utils::print_array(&get_peptide_vector_from_peptide(
        peptide.trim(),
        &get_aa_to_mass_usize(),
    ));
}

fn get_peptide_vector_from_peptide(peptide: &str, aa_to_mass: &HashMap<char, usize>) -> Vec<u8> {
    let prefix_masses = get_prefix_masses(peptide, aa_to_mass);
    let mut peptide_vector: Vec<_> = (0..prefix_masses[prefix_masses.len() - 1])
        .map(|_| 0u8)
        .collect();
    for m in prefix_masses[1..].iter() {
        peptide_vector[*m - 1] = 1;
    }
    peptide_vector
}
