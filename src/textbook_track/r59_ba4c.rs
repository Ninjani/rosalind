use crate::utils;
use std::collections::HashMap;

pub fn rosalind_ba4c() {
    let protein = utils::input_from_file("data/textbook_track/rosalind_ba4c.txt");
    let aa_to_mass = get_aa_to_mass_usize();
    utils::print_array(&get_theoretical_cyclic_spectrum(&protein, &aa_to_mass));
}

pub fn get_theoretical_cyclic_spectrum(
    protein: &str,
    aa_to_mass: &HashMap<char, usize>,
) -> Vec<usize> {
    let mut spectrum: Vec<_> = vec![0];
    for i in 1..protein.len() {
        let add = protein.chars().collect::<Vec<_>>()[protein.len() - i + 1..]
            .iter()
            .collect::<String>();
        let current_protein = format!("{}{}", add, protein);
        for chunk in utils::kmerize(&current_protein, i) {
            spectrum.push(chunk.chars().map(|c| aa_to_mass[&c]).sum::<usize>());
        }
    }
    spectrum.push(protein.chars().map(|c| aa_to_mass[&c]).sum::<usize>());
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
