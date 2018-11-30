use crate::textbook_track::r59_ba4c::get_aa_to_mass_usize;
use crate::textbook_track::r66_ba4j::get_prefix_masses;
use crate::utils;

/// Convert a Peptide into a Peptide Vector
///
/// Given: A peptide P.
///
/// Return: The peptide vector of P.
pub fn rosalind_ba11c() {
    let peptide = utils::input_from_file("data/textbook_track/rosalind_ba11c.txt");
    let aa_to_mass = get_aa_to_mass_usize();
    let peptide_masses: Vec<_> = peptide.trim().chars().map(|c| aa_to_mass[&c]).collect();
    utils::print_array(&get_peptide_vector_from_peptide(&peptide_masses));
}

fn get_peptide_vector_from_peptide(peptide: &[usize]) -> Vec<u8> {
    let prefix_masses = get_prefix_masses(peptide);
    let mut peptide_vector: Vec<_> = (0..prefix_masses[prefix_masses.len() - 1])
        .map(|_| 0u8)
        .collect();
    for m in prefix_masses[1..].iter() {
        peptide_vector[*m - 1] = 1;
    }
    peptide_vector
}
