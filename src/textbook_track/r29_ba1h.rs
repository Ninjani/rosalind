use crate::stronghold::r6_hamm::hamming;
use crate::utils;
use failure::Error;

pub fn rosalind_ba1h() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba1h.txt");
    let lines = contents.split('\n').collect::<Vec<_>>();
    let (pattern, text, mismatch) = (
        lines[0],
        lines[1],
        lines[2].parse::<usize>()?,
    );
    utils::print_array(&find_pattern_approx(text, pattern, mismatch));
    Ok(())
}

fn find_pattern_approx(text: &str, pattern: &str, mismatch: usize) -> Vec<usize> {
    let mut indices = Vec::new();
    let text: Vec<_> = text.chars().collect();
    for i in 0..=(text.len() - pattern.len()) {
        if hamming(
            &text[i..(i + pattern.len())].iter().collect::<String>(),
            pattern,
        ) <= mismatch
        {
            indices.push(i);
        }
    }
    indices
}
