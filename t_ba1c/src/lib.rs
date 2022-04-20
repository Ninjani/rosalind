use anyhow::Error;

use s_revc::reverse_complement;
use std::path::Path;

pub fn rosalind_ba1c(filename: &Path) -> Result<(), Error> {
    let dna = utility::io::input_from_file(filename)?;
    println!("{}", reverse_complement(&dna));
    Ok(())
}
