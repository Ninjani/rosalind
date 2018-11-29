use crate::utils;

/// Finding a motif in DNA
///
/// Given: Two DNA strings s and t (each of length at most 1 kbp).
///
/// Return: All locations of t as a substring of s.
pub fn rosalind_subs() {
    let contents = utils::input_from_file("data/stronghold/rosalind_subs.txt");
    let dna_motif = contents.split('\n').collect::<Vec<&str>>();
    let dna = dna_motif[0];
    let motif = dna_motif[1];
    utils::print_array(
        &utils::find_motifs(motif, dna)
            .into_iter()
            .map(|x| x + 1)
            .collect::<Vec<usize>>(),
    );
}
