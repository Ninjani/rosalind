use failure::Error;

use t_ba4c::get_aa_to_mass_usize;
use t_ba4g::score_linear_peptide;
use utility;
use utility::io::Parseable;

/// Compute the Score of a Linear Peptide
///
/// Given: An amino acid string Peptide and a collection of integers LinearSpectrum.
///
/// Return: The linear score of Peptide against Spectrum, LinearScore(Peptide, Spectrum).
pub fn rosalind_ba4k(filename: &str) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let lines: Vec<_> = contents.split('\n').collect();
    let (peptide, spectrum) = (lines[0], usize::parse_line(lines[1])?);
    let aa_to_mass = get_aa_to_mass_usize()?;
    let peptide_masses: Vec<_> = peptide.chars().map(|c| aa_to_mass[&c]).collect();
    println!("{}", score_linear_peptide(&peptide_masses, &spectrum));
    Ok(())
}
