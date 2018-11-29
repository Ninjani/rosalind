use crate::utils;

pub fn rosalind_ba1a() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba1a.txt");
    let mut lines = contents.split('\n');
    let (text, pattern) = (lines.next().unwrap(), lines.next().unwrap());
    println!("{}", pattern_count(&text, &pattern));
}

pub fn pattern_count(text: &str, pattern: &str) -> usize {
    let mut count = 0;
    let text: Vec<_> = text.chars().collect();
    let pattern: Vec<_> = pattern.chars().collect();
    for i in 0..=(text.len() - pattern.len()) {
        if text[i..(i + pattern.len())] == pattern[..] {
            count += 1;
        }
    }
    count
}
