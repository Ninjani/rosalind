use failure::Error;

use s_prot::translate;
use utility;

pub fn rosalind_ba4a(filename: &str) -> Result<(), Error> {
    let rna = utility::io::input_from_file(filename)?;
    let codons = utility::io::get_codon_to_aa()?;
    println!(
        "{}",
        translate(&rna, &codons).ok_or_else(|| utility::errors::RosalindOutputError::NoneError)?
    );
    Ok(())
}
