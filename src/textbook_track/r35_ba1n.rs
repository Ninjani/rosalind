use failure::Error;

use crate::textbook_track::r30_ba1i::get_mismatch_sequences;
use crate::utility;

pub fn rosalind_ba1n() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba1n.txt")?;
    let mut lines = contents.split('\n');
    let (pattern, mismatch) = (
        lines.next().unwrap(),
        lines.next().unwrap().parse::<usize>().unwrap(),
    );
    for neighbor in get_mismatch_sequences(&pattern, mismatch) {
        println!("{}", neighbor);
    }
    Ok(())
}
