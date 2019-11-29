use failure::Error;

use utility;

pub fn rosalind_ba1h() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba1h.txt")?;
    let lines = contents.split('\n').collect::<Vec<_>>();
    let (pattern, text, mismatch) = (lines[0], lines[1], lines[2].parse::<usize>()?);
    println!(
        "{}",
        utility::io::format_array(&find_pattern_approx(text, pattern, mismatch))
    );
    Ok(())
}

fn find_pattern_approx(text: &str, pattern: &str, mismatch: usize) -> Vec<usize> {
    let mut indices = Vec::new();
    let text: Vec<_> = text.chars().collect();
    for i in 0..=(text.len() - pattern.len()) {
        if utility::string::hamming(
            &text[i..(i + pattern.len())].iter().collect::<String>(),
            pattern,
        ) <= mismatch
        {
            indices.push(i);
        }
    }
    indices
}
