use crate::textbook_track::r66_ba4j::get_prefix_masses;
use crate::utils;
use hashbrown::HashMap;

pub fn rosalind_ba4c() {
    let peptide = utils::input_from_file("data/textbook_track/rosalind_ba4c.txt");
    let aa_to_mass = get_aa_to_mass_usize();
    let peptide_masses: Vec<_> = peptide.trim().chars().map(|c| aa_to_mass[&c]).collect();
    utils::print_array(&get_cyclic_spectrum(&peptide_masses));
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
    spectrum.sort();
    spectrum
}

const MASS_FILE: &str = "data/monoisotopic_mass.txt";

/// Reads monoisotopic mass table
pub fn get_aa_to_mass_usize() -> HashMap<char, usize> {
    let mut mass_table = HashMap::new();
    let mass_contents = utils::input_from_file(MASS_FILE);
    for line in mass_contents.split('\n') {
        let mut aa_mass = line.split_whitespace();
        if let (Some(aa), Some(mass)) = (aa_mass.next(), aa_mass.next()) {
            mass_table.insert(
                aa.chars().next().unwrap(),
                mass.parse::<f64>().unwrap().floor() as usize,
            );
        }
    }
    mass_table
}
