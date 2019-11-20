use failure::Error;

use crate::utility;

pub fn rosalind_ba1d() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba1d.txt")?;
    let lines = contents.split('\n').collect::<Vec<_>>();
    println!(
        "{}",
        utility::io::format_array(&find_pattern(lines[1], lines[0]))
    );
    Ok(())
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
