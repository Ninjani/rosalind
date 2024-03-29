use anyhow::Error;

use std::path::Path;

pub fn rosalind_ba1f(filename: &Path) -> Result<(), Error> {
    let genome = utility::io::input_from_file(filename)?;
    println!("{}", utility::io::format_array(&minimize_skews(&genome)));
    Ok(())
}

fn get_counts(text: &str, character: char) -> Vec<usize> {
    let presence: Vec<_> = text
        .chars()
        .map(|c| if c == character { 1 } else { 0 })
        .collect();
    let mut counts = Vec::new();
    let mut current_count = 0;
    for p in presence {
        current_count += p;
        counts.push(current_count);
    }
    counts
}

fn minimize_skews(text: &str) -> Vec<usize> {
    let mut skews = vec![(0, 0)];
    let g_counts = get_counts(text, 'G');
    let c_counts = get_counts(text, 'C');
    for i in 0..text.len() {
        skews.push((i, (g_counts[i] as isize) - (c_counts[i] as isize)));
    }
    skews.sort_by(|a, b| a.1.cmp(&b.1));
    let min_skew = skews[0].1;
    let mut indices = Vec::new();
    for (i, skew) in skews {
        if skew > min_skew {
            break;
        }
        indices.push(i + 1);
    }
    indices
}
