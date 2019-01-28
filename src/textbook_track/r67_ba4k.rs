use crate::textbook_track::r59_ba4c::get_aa_to_mass_usize;
use crate::textbook_track::r63_ba4g::score_linear_peptide;
use crate::utils;
use crate::utils::Parseable;
use failure::Error;

/// Compute the Score of a Linear Peptide
///
/// Given: An amino acid string Peptide and a collection of integers LinearSpectrum.
///
/// Return: The linear score of Peptide against Spectrum, LinearScore(Peptide, Spectrum).
pub fn rosalind_ba4k() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba4k.txt");
    let lines: Vec<_> = contents.split('\n').collect();
    let (peptide, spectrum) = (lines[0], usize::parse_line(lines[1])?);
    let aa_to_mass = get_aa_to_mass_usize();
    let peptide_masses: Vec<_> = peptide.chars().map(|c| aa_to_mass[&c]).collect();
    println!("{}", score_linear_peptide(&peptide_masses, &spectrum));
    Ok(())
}
