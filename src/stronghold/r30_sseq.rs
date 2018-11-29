use crate::utils;

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

/// Finding a Spliced Motif
///
/// Given: Two DNA strings s and t (each of length at most 1 kbp) in FASTA format.
///
/// Return: One collection of indices of s in which the symbols of t appear as a subsequence of s. If multiple solutions exist, you may return any one.
pub fn rosalind_sseq() {
    let fasta = utils::read_fasta_file("data/stronghold/rosalind_sseq.txt");
    let (_, sequence) = fasta
        .iter()
        .max_by(|(_, v), (_, v1)| v.len().cmp(&v1.len()))
        .unwrap();
    let (_, motif) = fasta
        .iter()
        .min_by(|(_, v), (_, v1)| v.len().cmp(&v1.len()))
        .unwrap();
    utils::print_array(&find_spliced_motif_positions(sequence, motif));
}
