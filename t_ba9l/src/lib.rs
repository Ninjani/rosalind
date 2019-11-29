use std::collections::HashMap;
use std::hash::Hash;

use failure::Error;

use crate::textbook_track::r118_ba9k::get_last_to_first_mapping;
use utility;

/// Implement BWMatching
///
/// Given: A string BWT(Text), followed by a collection of strings Patterns.
///
/// Return: A list of integers, where the i-th integer corresponds to the number of substring
/// matches of the i-th member of Patterns in Text.
pub fn rosalind_ba9l(filename: &str) -> Result<Vec<usize>, Error> {
    let content = utility::io::input_from_file(filename)?;
    let lines: Vec<_> = content.split('\n').collect();
    let bwt_text: Vec<_> = lines[0].chars().collect();
    let mut first_column = bwt_text.clone();
    first_column.sort();
    let mut counts = Vec::new();
    let last_to_first = get_last_to_first_mapping(&bwt_text, &first_column);
    for pattern in lines[1].split_whitespace() {
        let pattern: Vec<_> = pattern.chars().collect();
        counts.push(bwm_matching(
            &first_column,
            &bwt_text,
            &pattern,
            &last_to_first,
        ));
    }
    println!("{}", utility::io::format_array(&counts));
    Ok(counts)
}

fn bwm_matching<T: Hash + Eq>(
    first_column: &[T],
    last_column: &[T],
    pattern: &[T],
    last_to_first: &HashMap<usize, usize>,
) -> usize {
    let mut top = 0;
    let mut bottom = last_column.len() - 1;
    let mut pattern = pattern.into_iter().rev();
    while top <= bottom {
        if let Some(last_letter) = pattern.next() {
            let indices: Vec<_> = (top..=bottom)
                .filter(|i| &last_column[*i] == last_letter)
                .collect();
            if indices.is_empty() {
                return 0;
            } else {
                top = last_to_first[&indices[0]];
                bottom = last_to_first[&indices[indices.len() - 1]];
            }
        } else {
            return bottom - top + 1;
        }
    }
    0
}
