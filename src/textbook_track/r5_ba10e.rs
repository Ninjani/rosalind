use crate::textbook_track::hidden_markov_models::{ProfileHMM, ProfileHMMError, read_chars};
use crate::utils;
use failure::Error;

/// Construct a Profile HMM
///
/// Given: A threshold θ, followed by an alphabet Σ,
/// followed by a multiple alignment Alignment whose strings are formed from Σ.
///
/// Return: The transition and emission probabilities of the profile HMM HMM(Alignment, θ).
pub fn rosalind_ba10e() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba10e.txt");
    let mut sections = contents.split("--------");
    let threshold = sections
        .next()
        .ok_or_else(|| ProfileHMMError::InputFormatError("Missing threshold".into()))?
        .trim()
        .parse::<f32>()?;
    let (alphabet, alphabet_index) = read_chars(
        sections
            .next()
            .ok_or_else(|| ProfileHMMError::InputFormatError("Missing alphabet".into()))?,
    );
    let msa_section = sections
        .next()
        .ok_or_else(|| ProfileHMMError::InputFormatError("Missing alignment".into()))?;
    let hmm = ProfileHMM::new(threshold, None, alphabet, alphabet_index, msa_section)?;
    hmm.print_transition_matrix();
    println!("--------");
    hmm.print_emission_matrix();
    Ok(())
}
