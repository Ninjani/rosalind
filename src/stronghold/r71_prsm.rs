use crate::stronghold::r20_prtm::get_aa_to_mass;
use crate::stronghold::r55_conv::{get_max_multiplicity, get_minkowski_difference};
use crate::utils;
use std::collections::HashMap;
use failure::{err_msg, Error};

/// Matching a Spectrum to a Protein
///
/// Given: A positive integer n followed by a collection of n protein strings s1, s2, ..., sn and a
/// multiset R of positive numbers (corresponding to the complete spectrum of some unknown protein string).
///
/// Return: The maximum multiplicity of R⊖S[sk] taken over all strings sk, followed by the string sk
/// for which this maximum multiplicity occurs (you may output any such value if multiple solutions exist).
pub fn rosalind_prsm() -> Result<(), Error> {
    let aa_to_mass = get_aa_to_mass()?;
    let contents = utils::input_from_file("data/stronghold/rosalind_prsm.txt");
    let lines: Vec<_> = contents.split('\n').collect();
    let num_proteins = lines[0].parse::<usize>()?;
    let proteins: Vec<_> = lines[1..=num_proteins].iter().collect();
    let spectrum: Vec<_> = lines[num_proteins + 1..]
        .iter()
        .map(|x| x.parse::<f64>())
        .collect::<Result<Vec<_>, _>>()?;
    let (max_multiplicity, max_index) = proteins
        .iter()
        .enumerate()
        .map(|(i, protein)| {
            (
                get_max_multiplicity(&get_minkowski_difference(
                    &spectrum,
                    &get_complete_spectrum(protein, &aa_to_mass),
                )).unwrap()
                .0,
                i,
            )
        })
        .max()
        .ok_or(err_msg("NoneError"))?;
    println!("{}\n{}", max_multiplicity, proteins[max_index]);
    Ok(())
}

fn get_complete_spectrum(protein: &str, aa_to_mass: &HashMap<char, f64>) -> Vec<f64> {
    let protein: Vec<_> = protein.chars().collect();
    let mut spectrum = Vec::new();
    for i in 0..protein.len() {
        spectrum.push(protein[i..].iter().map(|c| aa_to_mass[&c]).sum::<f64>());
        spectrum.push(protein[..i].iter().map(|c| aa_to_mass[&c]).sum::<f64>());
    }
    spectrum
}
