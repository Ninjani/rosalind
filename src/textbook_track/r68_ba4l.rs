use crate::textbook_track::r59_ba4c::get_aa_to_mass_usize;
use crate::textbook_track::r63_ba4g::trim_leaderboard;
use crate::utils;
use crate::utils::Parseable;
use std::collections::{HashMap, HashSet};
use failure::Error;

/// Trim a Peptide Leaderboard
///
/// Given: A leaderboard of linear peptides Leaderboard, a linear spectrum Spectrum, and an integer N.
///
/// Return: The top N peptides from Leaderboard scored against Spectrum. Remember to use LinearScore.
pub fn rosalind_ba4l() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba4l.txt");
    let lines: Vec<_> = contents.split('\n').collect();
    let aa_to_mass = get_aa_to_mass_usize();
    let masses_to_peptide: HashMap<_, _> = lines[0]
        .split_whitespace()
        .map(|p| (p.chars().map(|c| aa_to_mass[&c]).collect::<Vec<_>>(), p))
        .collect();
    let leaderboard: HashSet<_> = masses_to_peptide.keys().cloned().collect();
    let (spectrum, n) = (
        usize::parse_line(lines[1])?,
        lines[2].parse::<usize>()?,
    );
    utils::print_array(
        &trim_leaderboard(&leaderboard, &spectrum, n)
            .into_iter()
            .map(|masses| masses_to_peptide[&masses])
            .collect::<Vec<_>>(),
    );
    Ok(())
}
