use crate::textbook_track::{
    hidden_markov_models::{HMMError, HMM},
    r5_ba10e::ProfileHMM,
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
        .ok_or(HMMError::FormatError(
            "Missing threshold/pseudocount".into(),
        ))?
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<f32>())
        .collect::<Result<Vec<_>, _>>()?;
    let (threshold, pseudocount) = (threshold_pseudocount[0], threshold_pseudocount[1]);
    let (alphabet, alphabet_index) = HMM::read_chars(
        sections
            .next()
            .ok_or(HMMError::FormatError("Missing alphabet".into()))?,
    );
    let msa_section = sections
        .next()
        .ok_or(HMMError::FormatError("Missing alignment".into()))?;
    let hmm = ProfileHMM::new(
        threshold,
        Some(pseudocount),
        alphabet,
        alphabet_index,
        msa_section,
    );
    hmm.print_transitions();
    println!("--------");
    hmm.print_emissions();
    println!("{:?}", hmm);
    Ok(())
}
