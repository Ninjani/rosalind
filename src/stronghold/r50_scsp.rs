use crate::stronghold::r38_lcsq::longest_common_subsequence;
use crate::utils;

/// Interleaving Two Motifs
///
/// Given: Two DNA strings s and t.
///
/// Return: A shortest common supersequence of s and t. If multiple solutions exist, you may output any one.
pub fn rosalind_scsp() {
    let contents = utils::input_from_file("data/stronghold/rosalind_scsp.txt");
    let sequences: Vec<_> = contents.split('\n').collect();
    println!(
        "{}",
        shortest_common_supersequence(sequences[0], sequences[1])
    );
}

pub fn shortest_common_supersequence(string_1: &str, string_2: &str) -> String {
    let lcs = longest_common_subsequence(string_1, string_2);
    let mut scs = String::new();
    let mut chars_1: Vec<_> = string_1.chars().collect();
    let mut chars_2: Vec<_> = string_2.chars().collect();
    let mut lcs_chars: Vec<_> = lcs.chars().collect();
    while !lcs_chars.is_empty() {
        if chars_1[0] == lcs_chars[0] && chars_2[0] == lcs_chars[0] {
            scs.push(lcs_chars[0]);
            lcs_chars = lcs_chars[1..].to_vec();
            chars_1 = chars_1[1..].to_vec();
            chars_2 = chars_2[1..].to_vec();
        } else if chars_1[0] == lcs_chars[0] {
            scs.push(chars_2[0]);
            chars_2 = chars_2[1..].to_vec();
        } else {
            scs.push(chars_1[0]);
            chars_1 = chars_1[1..].to_vec();
        }
    }
    format!(
        "{}{}{}",
        scs,
        chars_1.into_iter().collect::<String>(),
        chars_2.into_iter().collect::<String>()
    )
}
