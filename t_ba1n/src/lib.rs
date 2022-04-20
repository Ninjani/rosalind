use anyhow::Error;

use std::path::Path;
use t_ba1i::get_mismatch_sequences;

pub fn rosalind_ba1n(filename: &Path) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let mut lines = contents.split('\n');
    let (pattern, mismatch) = (
        lines.next().unwrap(),
        lines.next().unwrap().parse::<usize>().unwrap(),
    );
    for neighbor in get_mismatch_sequences(pattern, mismatch) {
        println!("{}", neighbor);
    }
    Ok(())
}
