use failure::Error;

use utility;

pub fn rosalind_ba1g(filename: &str) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let lines = contents.split('\n').collect::<Vec<_>>();
    println!("{}", utility::string::hamming(lines[0], lines[1]));
    Ok(())
}
