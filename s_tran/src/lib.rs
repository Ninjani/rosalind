use failure::Error;

use crate::utility;

/// Transitions and Transversions
///
/// Given: Two DNA strings s1 and s2 of equal length (at most 1 kbp).
///
/// Return: The transition/transversion ratio R(s1,s2).
pub fn rosalind_tran(filename: &str) -> Result<f64, Error> {
    let fasta = utility::io::read_fasta_file(filename)?;
    let sequences = fasta.values().collect::<Vec<_>>();
    let sequence_1 = sequences[0];
    let sequence_2 = sequences[1];
    let ratio = (count_transitions(sequence_1, sequence_2) as f64)
        / (count_transversions(sequence_1, sequence_2)) as f64;
    println!("{}", ratio);
    Ok(ratio)
}

/// Check if two nucleotides represent a transition
fn is_transition(c1: char, c2: char) -> bool {
    (c1, c2) == ('A', 'G')
        || (c1, c2) == ('G', 'A')
        || (c1, c2) == ('C', 'T')
        || (c1, c2) == ('T', 'C')
}

/// Check if two nucleotides represent a transversion
fn is_transversion(c1: char, c2: char) -> bool {
    c1 != c2 && !is_transition(c1, c2)
}

/// Count number of transitions in two equal-length sequences
fn count_transitions(sequence_1: &str, sequence_2: &str) -> usize {
    sequence_1
        .chars()
        .zip(sequence_2.chars())
        .filter(|(c1, c2)| is_transition(*c1, *c2))
        .count()
}

/// Count number of transversions in two equal-length sequences
fn count_transversions(sequence_1: &str, sequence_2: &str) -> usize {
    sequence_1
        .chars()
        .zip(sequence_2.chars())
        .filter(|(c1, c2)| is_transversion(*c1, *c2))
        .count()
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn tran() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_tran")?;
        assert_approx_eq!(
            rosalind_tran(&input_file)?,
            utility::io::input_from_file(&output_file)?.parse::<f64>()?,
            utility::testing::ROSALIND_FLOAT_ERROR_F64
        );
        Ok(())
    }
}
