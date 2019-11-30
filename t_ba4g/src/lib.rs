use std::collections::HashSet;
use std::iter::FromIterator;

use failure::Error;
use itertools::Itertools;

use t_ba4c::get_aa_to_mass_usize;
use t_ba4e::{get_linear_spectrum, expand, spectrum_list_to_counts};
use t_ba4f::score_cyclic_peptide;
use utility;
use utility::io::Parseable;

/// Implement LeaderboardCyclopeptideSequencing
///
/// Given: An integer N and a collection of integers Spectrum.
///
/// Return: LeaderPeptide after running LeaderboardCyclopeptideSequencing(Spectrum, N).
pub fn rosalind_ba4g(filename: &str) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let lines: Vec<_> = contents.split('\n').collect();
    let (n, spectrum) = (lines[0].parse::<usize>()?, usize::parse_line(lines[1])?);
    let aa_to_mass = get_aa_to_mass_usize()?;
    let masses: HashSet<_> = aa_to_mass.values().cloned().collect();
    let peptide =
        leaderboard_cyclo_peptide_sequencing(&spectrum, n, &masses.into_iter().collect::<Vec<_>>())
            .into_iter()
            .map(|mass| mass.to_string())
            .collect::<Vec<_>>()
            .join("-");
    println!("{}", peptide);
    Ok(())
}

pub fn leaderboard_cyclo_peptide_sequencing(
    spectrum: &[usize],
    n: usize,
    amino_acid_masses: &[usize],
) -> Vec<usize> {
    let mut leaderboard = HashSet::new();
    let mut leaderpeptide = Vec::new();
    leaderboard.insert(Vec::new());
    let parent_mass = *spectrum.iter().max().unwrap();
    while !leaderboard.is_empty() {
        leaderboard = expand(&leaderboard, amino_acid_masses);
        for peptide in &leaderboard.clone() {
            let mass = peptide.iter().sum::<usize>();
            if mass == parent_mass
                && score_cyclic_peptide(peptide, spectrum)
                > score_cyclic_peptide(&leaderpeptide, spectrum)
            {
                leaderpeptide = peptide.clone();
            } else if mass > parent_mass {
                leaderboard.remove(peptide);
            }
        }
        leaderboard = HashSet::from_iter(trim_leaderboard(&leaderboard, spectrum, n).into_iter());
    }
    leaderpeptide
}

pub fn score_linear_peptide(peptide: &[usize], spectrum: &[usize]) -> usize {
    let linear_spectrum_counts = spectrum_list_to_counts(&get_linear_spectrum(peptide));
    let spectrum_counts = spectrum_list_to_counts(spectrum);
    linear_spectrum_counts
        .into_iter()
        .filter(|(mass, _)| spectrum_counts.contains_key(mass))
        .map(|(mass, count)| count.min(spectrum_counts[&mass]))
        .sum::<usize>()
}

pub fn get_top_with_ties<T: Clone, U: Eq + Ord>(item_scores: &[(T, U)], n: usize) -> Vec<T> {
    let item_scores: Vec<_> = item_scores.iter().sorted_by(|a, b| b.1.cmp(&a.1)).collect();
    for j in n..item_scores.len() {
        if item_scores[j].1 < item_scores[n - 1].1 {
            return (0..j).map(|x| item_scores[x].0.clone()).collect();
        }
    }
    item_scores.into_iter().map(|(x, _)| x.clone()).collect()
}

pub fn trim_leaderboard(
    leaderboard: &HashSet<Vec<usize>>,
    spectrum: &[usize],
    n: usize,
) -> Vec<Vec<usize>> {
    let peptide_scores: Vec<_> = leaderboard
        .iter()
        .map(|peptide| (peptide.clone(), score_linear_peptide(peptide, spectrum)))
        .collect();
    get_top_with_ties(&peptide_scores, n)
}
