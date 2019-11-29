use failure::Error;

use crate::stronghold::r46_edit::get_edit_distance;
use utility;

pub fn rosalind_ba5g() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba5g.txt")?;
    let lines: Vec<_> = contents.split('\n').collect();
    println!("{}", get_edit_distance(lines[0], lines[1]));
    Ok(())
}
