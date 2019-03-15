use crate::textbook_track::r59_ba4c::get_aa_to_mass_usize;
use crate::textbook_track::r59_ba4c::get_cyclic_spectrum;
use crate::textbook_track::r66_ba4j::get_linear_spectrum;
use crate::utils;
use crate::utils::Parseable;
use hashbrown::{HashMap, HashSet};
use failure::Error;

/// Find a Cyclic Peptide with Theoretical Spectrum Matching an Ideal Spectrum
///
/// Given: A collection of (possibly repeated) integers Spectrum corresponding to an ideal experimental spectrum.
///
/// Return: Every amino acid string Peptide such that Cyclospectrum(Peptide) = Spectrum (if such a string exists).
pub fn rosalind_ba4e() -> Result<(), Error> {
    let spectrum = usize::parse_line(&utils::input_from_file(
        "data/textbook_track/rosalind_ba4e.txt",
    ))?;
    let aa_to_mass = &get_aa_to_mass_usize();
    let masses: HashSet<_> = aa_to_mass.values().cloned().collect();
    let peptides: Vec<_> =
        cyclo_peptide_sequencing(&spectrum, &masses.into_iter().collect::<Vec<_>>())
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
    Ok(())
}

pub fn expand(peptides: &HashSet<Vec<usize>>, amino_acid_masses: &[usize]) -> HashSet<Vec<usize>> {
    let mut expanded_peptides = HashSet::new();
    for peptide in peptides {
        for mass in amino_acid_masses {
            let mut new_peptide = peptide.clone();
            new_peptide.push(*mass);
            expanded_peptides.insert(new_peptide);
        }
    }
    expanded_peptides
}

pub fn spectrum_list_to_counts(spectrum: &[usize]) -> HashMap<usize, usize> {
    let mut spectrum_counts = HashMap::new();
    for mass in spectrum {
        *spectrum_counts.entry(*mass).or_insert(0usize) += 1;
    }
    spectrum_counts
}

pub fn is_consistent(peptide: &[usize], spectrum: &[usize]) -> bool {
    let peptide_spectrum = get_linear_spectrum(peptide);
    let mut spectrum_counts = spectrum_list_to_counts(spectrum);
    for mass in peptide_spectrum {
        match spectrum_counts.get_mut(&mass) {
            Some(count) => *count -= 1,
            None => return false,
        }
    }
    true
}

fn cyclo_peptide_sequencing(
    spectrum: &[usize],
    amino_acid_masses: &[usize],
) -> HashSet<Vec<usize>> {
    let mut peptides = HashSet::new();
    peptides.insert(Vec::new());
    let parent_mass = *spectrum.iter().max().unwrap();
    let mut cyclopeptides = HashSet::new();
    while !peptides.is_empty() {
        peptides = expand(&peptides, amino_acid_masses);
        for peptide in &peptides.clone() {
            if peptide.iter().sum::<usize>() == parent_mass {
                if get_cyclic_spectrum(peptide) == spectrum {
                    cyclopeptides.insert(peptide.clone());
                }
                peptides.remove(peptide);
            } else if !is_consistent(peptide, spectrum) {
                peptides.remove(peptide);
            }
        }
    }
    cyclopeptides
}
