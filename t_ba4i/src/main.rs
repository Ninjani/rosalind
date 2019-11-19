use failure::Error;

use crate::textbook_track::r61_ba4e::spectrum_list_to_counts;
use crate::textbook_track::r63_ba4g::{get_top_with_ties, leaderboard_cyclo_peptide_sequencing};
use crate::textbook_track::r64_ba4h::get_spectral_convolution;
use crate::utility;
use crate::utility::io::Parseable;

/// Implement ConvolutionCyclopeptideSequencing
///
/// Given: An integer M, an integer N, and a collection of (possibly repeated) integers Spectrum.
///
/// Return: A cyclic peptide LeaderPeptide with amino acids taken only from the top M elements
/// (and ties) of the convolution of Spectrum that fall between 57 and 200, and where the size
/// of Leaderboard is restricted to the top N (and ties).
pub fn rosalind_ba4i() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba4i.txt")?;
    let lines: Vec<_> = contents.split('\n').collect();
    let (m, n, spectrum) = (
        lines[0].parse::<usize>()?,
        lines[1].parse::<usize>()?,
        usize::parse_line(lines[2])?,
    );
    let peptide = convolution_cyclo_peptide_sequencing(&spectrum, m, n)
        .into_iter()
        .map(|mass| mass.to_string())
        .collect::<Vec<_>>()
        .join("-");
    println!("{}", peptide);
    Ok(())
}

fn convolution_cyclo_peptide_sequencing(spectrum: &[usize], m: usize, n: usize) -> Vec<usize> {
    let convolution_counts: Vec<_> = spectrum_list_to_counts(&get_spectral_convolution(spectrum))
        .into_iter()
        .filter(|(mass, _)| *mass >= 57 && *mass <= 200)
        .collect();
    let amino_acid_masses = get_top_with_ties(&convolution_counts, m);
    leaderboard_cyclo_peptide_sequencing(spectrum, n, &amino_acid_masses)
}
