use crate::stronghold::r6_hamm::hamming;
use crate::utils;

pub fn rosalind_ba1h() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba1h.txt");
    let mut lines = contents.split('\n');
    let (pattern, text, mismatch) = (
        lines.next().unwrap(),
        lines.next().unwrap(),
        lines.next().unwrap().parse::<usize>().unwrap(),
    );
    utils::print_array(&find_pattern_approx(&text, &pattern, mismatch));
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
