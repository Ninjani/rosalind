use anyhow::Error;

use std::path::Path;
use t_ba4c::get_aa_to_mass_usize;
use t_ba4e::get_linear_spectrum;

/// Generate the Theoretical Spectrum of a Linear Peptide
///
/// Given: An amino acid string Peptide.
///
/// Return: The linear spectrum of Peptide.
pub fn rosalind_ba4j(filename: &Path) -> Result<(), Error> {
    let peptide = utility::io::input_from_file(filename)?;
    let aa_to_mass = get_aa_to_mass_usize()?;
    let peptide_masses: Vec<_> = peptide.trim().chars().map(|c| aa_to_mass[&c]).collect();
    println!(
        "{}",
        utility::io::format_array(&get_linear_spectrum(&peptide_masses))
    );
    Ok(())
}
