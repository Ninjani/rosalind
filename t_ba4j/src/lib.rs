use failure::Error;

use crate::textbook_track::r59_ba4c::get_aa_to_mass_usize;
use utility;

/// Generate the Theoretical Spectrum of a Linear Peptide
///
/// Given: An amino acid string Peptide.
///
/// Return: The linear spectrum of Peptide.
pub fn rosalind_ba4j() -> Result<(), Error> {
    let peptide = utility::io::input_from_file("data/textbook_track/rosalind_ba4j.txt")?;
    let aa_to_mass = get_aa_to_mass_usize()?;
    let peptide_masses: Vec<_> = peptide.trim().chars().map(|c| aa_to_mass[&c]).collect();
    println!(
        "{}",
        utility::io::format_array(&get_linear_spectrum(&peptide_masses))
    );
    Ok(())
}

pub fn get_prefix_masses(peptide: &[usize]) -> Vec<usize> {
    let mut prefix_masses = vec![0];
    for i in 1..=peptide.len() {
        let previous_mass = prefix_masses[i - 1];
        prefix_masses.push(previous_mass + peptide[i - 1])
    }
    prefix_masses
}

pub fn get_linear_spectrum(peptide: &[usize]) -> Vec<usize> {
    let prefix_masses = get_prefix_masses(peptide);
    let mut spectrum = vec![0];
    for i in 0..peptide.len() {
        for j in (i + 1)..=peptide.len() {
            spectrum.push(prefix_masses[j] - prefix_masses[i]);
        }
    }
    spectrum.sort();
    spectrum
}
