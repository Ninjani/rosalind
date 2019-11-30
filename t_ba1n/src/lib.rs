use failure::Error;

use t_ba1i::get_mismatch_sequences;
use utility;

pub fn rosalind_ba1n(filename: &str) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
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
