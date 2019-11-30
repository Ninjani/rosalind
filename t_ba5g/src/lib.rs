use failure::Error;

use s_edit::get_edit_distance;
use utility;

pub fn rosalind_ba5g(filename: &str) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let lines: Vec<_> = contents.split('\n').collect();
    println!("{}", get_edit_distance(lines[0], lines[1]));
    Ok(())
}
