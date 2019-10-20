use failure::Error;

use crate::utility;

pub fn rosalind_ba1g() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba1g.txt")?;
    let lines = contents.split('\n').collect::<Vec<_>>();
    println!("{}", utility::string::hamming(lines[0], lines[1]));
    Ok(())
}
