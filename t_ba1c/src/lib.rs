use failure::Error;

use s_revc::reverse_complement;
use utility;

pub fn rosalind_ba1c(filename: &str) -> Result<(), Error> {
    let dna = utility::io::input_from_file(filename)?;
    println!("{}", reverse_complement(&dna));
    Ok(())
}
