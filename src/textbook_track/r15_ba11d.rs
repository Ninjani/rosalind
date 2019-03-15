use crate::textbook_track::r12_ba11a::get_mass_to_aa;
use crate::utils;
use crate::utils::Parseable;
use hashbrown::HashMap;
use failure::Error;

/// Convert a Peptide Vector into a Peptide
///
/// Given: A space-delimited binary vector P.
///
/// Return: A peptide whose binary peptide vector matches P. For masses with more than one amino acid, any choice may be used.
pub fn rosalind_ba11d() -> Result<(), Error> {
    let peptide_vector = u8::parse_line(&utils::input_from_file(
        "data/textbook_track/rosalind_ba11d.txt",
    ))?;
    println!(
        "{}",
        get_peptide_from_peptide_vector(&peptide_vector, &get_mass_to_aa())
    );
    Ok(())
}

fn get_peptide_from_peptide_vector(
    peptide_vector: &[u8],
    mass_to_aa: &HashMap<usize, char>,
) -> String {
    let prefix_masses: Vec<_> = peptide_vector
        .iter()
        .enumerate()
        .filter(|(_, m)| **m == 1)
        .map(|(i, _)| i + 1)
        .collect();
    let mut peptide = mass_to_aa[&prefix_masses[0]].to_string();
    for i in 1..prefix_masses.len() {
        peptide.push(mass_to_aa[&(prefix_masses[i] - prefix_masses[i - 1])]);
    }
    peptide
}
