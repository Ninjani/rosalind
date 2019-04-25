use crate::textbook_track::{
    hidden_markov_models::{ProfileHMMError, ProfileHMM, read_chars},
};
use crate::utils;
use failure::Error;

/// W.I.P
/// Construct a Profile HMM
///
/// Given: A threshold θ, followed by an alphabet Σ,
/// followed by a multiple alignment Alignment whose strings are formed from Σ.
///
/// Return: The transition and emission probabilities of the profile HMM HMM(Alignment, θ).
pub fn rosalind_ba10f() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba10f.txt");
    let mut sections = contents.split("--------");
    let threshold_pseudocount = sections
        .next()
        .ok_or_else(|| ProfileHMMError::InputFormatError(
            "Missing threshold/pseudocount".into(),
        ))?
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<f32>())
        .collect::<Result<Vec<_>, _>>()?;
    let (threshold, pseudocount) = (threshold_pseudocount[0], threshold_pseudocount[1]);
    let (alphabet, alphabet_index) = read_chars(
        sections
            .next()
            .ok_or_else(|| ProfileHMMError::InputFormatError("Missing alphabet".into()))?,
    );
    let msa_section = sections
        .next()
        .ok_or_else(|| ProfileHMMError::InputFormatError("Missing alignment".into()))?;
    let hmm = ProfileHMM::new(
        threshold,
        Some(pseudocount),
        alphabet,
        alphabet_index,
        msa_section,
    )?;
    hmm.print_transition_matrix();
    println!("--------");
    hmm.print_emission_matrix();
    Ok(())
}
