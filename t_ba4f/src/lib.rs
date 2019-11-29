use failure::Error;

use crate::textbook_track::r59_ba4c::{get_aa_to_mass_usize, get_cyclic_spectrum};
use crate::textbook_track::r61_ba4e::spectrum_list_to_counts;
use utility;
use utility::io::Parseable;

/// Compute the Score of a Cyclic Peptide Against a Spectrum
///
/// Given: An amino acid string Peptide and a collection of integers Spectrum.
///
/// Return: The score of Peptide against Spectrum, Score(Peptide, Spectrum).
pub fn rosalind_ba4f() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba4f.txt")?;
    let lines: Vec<_> = contents.split('\n').collect();
    let (peptide, spectrum) = (lines[0], usize::parse_line(lines[1])?);
    let aa_to_mass = get_aa_to_mass_usize()?;
    let peptide_masses: Vec<_> = peptide.chars().map(|c| aa_to_mass[&c]).collect();
    println!("{}", score_cyclic_peptide(&peptide_masses, &spectrum));
    Ok(())
}

pub fn score_cyclic_peptide(peptide: &[usize], spectrum: &[usize]) -> usize {
    let cyclic_spectrum_counts = spectrum_list_to_counts(&get_cyclic_spectrum(peptide));
    let spectrum_counts = spectrum_list_to_counts(spectrum);
    cyclic_spectrum_counts
        .into_iter()
        .filter(|(mass, _)| spectrum_counts.contains_key(mass))
        .map(|(mass, count)| count.min(spectrum_counts[&mass]))
        .sum::<usize>()
}
