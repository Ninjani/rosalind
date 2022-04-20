use anyhow::Error;

use std::path::Path;

pub fn rosalind_ba1g(filename: &Path) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let lines = contents.split('\n').collect::<Vec<_>>();
    println!("{}", utility::string::hamming(lines[0], lines[1]));
    Ok(())
}
