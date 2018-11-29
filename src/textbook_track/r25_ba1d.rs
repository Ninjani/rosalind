use crate::utils;

pub fn rosalind_ba1d() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba1d.txt");
    let mut lines = contents.split('\n');
    let (pattern, text) = (lines.next().unwrap(), lines.next().unwrap());
    utils::print_array(&find_pattern(&text, &pattern));
}

fn find_pattern(text: &str, pattern: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    let text: Vec<_> = text.chars().collect();
    let pattern: Vec<_> = pattern.chars().collect();
    for i in 0..=(text.len() - pattern.len()) {
        if text[i..(i + pattern.len())] == pattern[..] {
            indices.push(i);
        }
    }
    indices
}
