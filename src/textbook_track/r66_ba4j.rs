use crate::textbook_track::r59_ba4c::get_aa_to_mass_usize;
use crate::utils;
use std::collections::HashMap;

/// Generate the Theoretical Spectrum of a Linear Peptide
///
/// Given: An amino acid string Peptide.
///
/// Return: The linear spectrum of Peptide.
pub fn rosalind_ba4j() {
    let peptide = utils::input_from_file("data/textbook_track/rosalind_ba4j.txt");
    utils::print_array(&get_theoretical_spectrum(
        peptide.trim(),
        &get_aa_to_mass_usize(),
    ));
}

pub fn get_prefix_masses(peptide: &str, aa_to_mass: &HashMap<char, usize>) -> Vec<usize> {
    let peptide_chars: Vec<_> = peptide.chars().collect();
    let mut prefix_masses = vec![0];
    for i in 1..=peptide.len() {
        let previous_mass = prefix_masses[i - 1];
        prefix_masses.push(previous_mass + aa_to_mass[&peptide_chars[i - 1]])
    }
    prefix_masses
}

pub fn get_theoretical_spectrum(peptide: &str, aa_to_mass: &HashMap<char, usize>) -> Vec<usize> {
    let prefix_masses = get_prefix_masses(peptide, aa_to_mass);
    let mut spectrum = vec![0];
    for i in 0..peptide.len() {
        for j in (i + 1)..=peptide.len() {
            spectrum.push(prefix_masses[j] - prefix_masses[i]);
        }
    }
    spectrum.sort();
    spectrum
}
