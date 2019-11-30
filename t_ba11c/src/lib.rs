use failure::Error;

use t_ba4c::{get_prefix_masses, get_aa_to_mass_usize};
use utility;

/// Convert a Peptide into a Peptide Vector
///
/// Given: A peptide P.
///
/// Return: The peptide vector of P.
pub fn rosalind_ba11c(filename: &str) -> Result<(), Error> {
    let peptide = utility::io::input_from_file(filename)?;
    let aa_to_mass = get_aa_to_mass_usize()?;
    let peptide_masses: Vec<_> = peptide.trim().chars().map(|c| aa_to_mass[&c]).collect();
    println!(
        "{}",
        utility::io::format_array(&get_peptide_vector_from_peptide(&peptide_masses))
    );
    Ok(())
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
