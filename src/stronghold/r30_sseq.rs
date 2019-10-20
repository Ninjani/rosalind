use failure::Error;

use crate::utility;

/// Finding a Spliced Motif
///
/// Given: Two DNA strings s and t (each of length at most 1 kbp) in FASTA format.
///
/// Return: One collection of indices of s in which the symbols of t appear as a subsequence of s.
/// If multiple solutions exist, you may return any one.
pub fn rosalind_sseq(filename: &str) -> Result<Vec<usize>, Error> {
    let fasta = utility::io::read_fasta_file(filename)?;
    let (_, sequence) = fasta
        .iter()
        .max_by(|(_, v), (_, v1)| v.len().cmp(&v1.len()))
        .unwrap();
    let (_, motif) = fasta
        .iter()
        .min_by(|(_, v), (_, v1)| v.len().cmp(&v1.len()))
        .unwrap();
    let indices = find_spliced_motif_positions(sequence, motif);
    println!("{}", utility::io::format_array(&indices));
    Ok(indices)
}

/// Finds positions of motif in sequence (non-contiguous)
fn find_spliced_motif_positions(sequence: &str, motif: &str) -> Vec<usize> {
    let mut motif_chars = motif.chars().peekable();
    let mut positions = Vec::new();
    let mut motif_char = *motif_chars.peek().unwrap();
    for (current_pos, current_char) in sequence.chars().enumerate() {
        if current_char == motif_char {
            positions.push(current_pos + 1);
            motif_chars.next().unwrap();
            match motif_chars.peek() {
                Some(character) => motif_char = *character,
                None => break,
            }
        }
    }
    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sseq() -> Result<(), Error> {
        let input_file = utility::testing::get_input_file("rosalind_sseq")?;
        let fasta = utility::io::read_fasta_file(&input_file)?;
        let (_, sequence) = fasta
            .iter()
            .max_by(|(_, v), (_, v1)| v.len().cmp(&v1.len()))
            .unwrap();
        let (_, motif) = fasta
            .iter()
            .min_by(|(_, v), (_, v1)| v.len().cmp(&v1.len()))
            .unwrap();
        let indices = rosalind_sseq(&input_file)?;
        let sequence_chars: Vec<_> = sequence.chars().collect();
        assert_eq!(
            &indices
                .into_iter()
                .map(|i| sequence_chars[i - 1])
                .collect::<String>(),
            motif
        );
        Ok(())
    }
}
