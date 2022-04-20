use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Error;

pub fn rosalind_ba4c(filename: &Path) -> Result<(), Error> {
    let peptide = utility::io::input_from_file(filename)?;
    let aa_to_mass = get_aa_to_mass_usize()?;
    let peptide_masses: Vec<_> = peptide.trim().chars().map(|c| aa_to_mass[&c]).collect();
    println!(
        "{}",
        utility::io::format_array(&get_cyclic_spectrum(&peptide_masses))
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

pub fn get_cyclic_spectrum(peptide: &[usize]) -> Vec<usize> {
    let prefix_masses = get_prefix_masses(peptide);
    let peptide_mass = prefix_masses[peptide.len()];
    let mut spectrum = vec![0];
    for i in 0..peptide.len() {
        for j in (i + 1)..=peptide.len() {
            spectrum.push(prefix_masses[j] - prefix_masses[i]);
            if i > 0 && j < peptide.len() {
                spectrum.push(peptide_mass - (prefix_masses[j] - prefix_masses[i]));
            }
        }
    }
    spectrum.sort_unstable();
    spectrum
}

/// Reads monoisotopic mass table
pub fn get_aa_to_mass_usize() -> Result<HashMap<char, usize>, Error> {
    let mut mass_table = HashMap::new();
    let mass_file: PathBuf = [env!("CARGO_WORKSPACE_DIR"), utility::io::MASS_FILE]
        .iter()
        .collect();
    let mass_contents = utility::io::input_from_file(&mass_file)?;
    for line in mass_contents.split('\n') {
        let mut aa_mass = line.split_whitespace();
        if let (Some(aa), Some(mass)) = (aa_mass.next(), aa_mass.next()) {
            mass_table.insert(
                aa.chars().next().unwrap(),
                mass.parse::<f64>()?.floor() as usize,
            );
        }
    }
    Ok(mass_table)
}
