use anyhow::Error;

use std::path::Path;

pub fn rosalind_ba1a(filename: &Path) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let lines = contents.split('\n').collect::<Vec<_>>();
    let (text, pattern) = (lines[0], lines[1]);
    println!("{}", pattern_count(text, pattern));
    Ok(())
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
