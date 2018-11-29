use crate::textbook_track::r59_ba4c::get_aa_to_mass_usize;
use crate::textbook_track::r59_ba4c::get_theoretical_cyclic_spectrum;
use crate::textbook_track::r66_ba4j::get_theoretical_spectrum;
use crate::utils;
use crate::utils::Parseable;
use std::collections::{HashMap, HashSet};

/// Find a Cyclic Peptide with Theoretical Spectrum Matching an Ideal Spectrum
///
/// Given: A collection of (possibly repeated) integers Spectrum corresponding to an ideal experimental spectrum.
///
/// Return: Every amino acid string Peptide such that Cyclospectrum(Peptide) = Spectrum (if such a string exists).
pub fn rosalind_ba4e() {
    let spectrum = usize::parse_line(&utils::input_from_file(
        "data/textbook_track/rosalind_ba4e.txt",
    ))
    .unwrap();
    let peptides: Vec<_> = cyclo_peptide_sequencing(&spectrum)
        .into_iter()
        .map(|masses| {
            masses
                .into_iter()
                .map(|mass| mass.to_string())
                .collect::<Vec<_>>()
                .join("-")
        })
        .collect();
    utils::print_array(&peptides);
}

fn expand(peptides: &HashSet<String>, amino_acids: &[char]) -> HashSet<String> {
    let mut expanded_peptides = HashSet::new();
    for peptide in peptides {
        for amino_acid in amino_acids {
            expanded_peptides.insert(format!("{}{}", peptide, amino_acid));
        }
    }
    expanded_peptides
}

fn get_mass(peptide: &str, aa_to_mass: &HashMap<char, usize>) -> usize {
    peptide.chars().map(|a| aa_to_mass[&a]).sum::<usize>()
}

fn is_consistent(peptide: &str, spectrum: &[usize], aa_to_mass: &HashMap<char, usize>) -> bool {
    let peptide_spectrum = get_theoretical_spectrum(peptide, aa_to_mass);
    let mut spectrum_counts = HashMap::new();
    for mass in spectrum {
        *spectrum_counts.entry(mass).or_insert(0) += 1;
    }
    for mass in peptide_spectrum {
        match spectrum_counts.get_mut(&mass) {
            Some(count) => *count -= 1,
            None => return false,
        }
    }
    true
}

fn cyclo_peptide_sequencing(spectrum: &[usize]) -> HashSet<Vec<usize>> {
    let aa_to_mass = &get_aa_to_mass_usize();
    let amino_acids: Vec<_> = aa_to_mass.keys().cloned().collect();
    let mut peptides = HashSet::new();
    peptides.insert(String::from(""));
    let parent_mass = *spectrum.iter().max().unwrap();
    let mut cyclopeptides = HashSet::new();
    while !peptides.is_empty() {
        peptides = expand(&peptides, &amino_acids);
        for peptide in &peptides.clone() {
            if get_mass(peptide, aa_to_mass) == parent_mass {
                if get_theoretical_cyclic_spectrum(peptide, aa_to_mass) == spectrum {
                    cyclopeptides.insert(peptide.chars().map(|c| aa_to_mass[&c]).collect());
                }
                peptides.remove(peptide);
            } else if !is_consistent(peptide, spectrum, aa_to_mass) {
                peptides.remove(peptide);
            }
        }
    }
    cyclopeptides
}
