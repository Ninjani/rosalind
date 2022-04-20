use anyhow::Error;

use s_prot::translate;
use std::path::Path;

pub fn rosalind_ba4a(filename: &Path) -> Result<(), Error> {
    let rna = utility::io::input_from_file(filename)?;
    let codons = utility::io::get_codon_to_aa()?;
    println!(
        "{}",
        translate(&rna, &codons).ok_or(utility::errors::RosalindOutputError::NoneError)?
    );
    Ok(())
}
