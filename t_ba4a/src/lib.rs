use failure::Error;

use crate::stronghold::r8_prot::translate;
use crate::utility;

pub fn rosalind_ba4a() -> Result<(), Error> {
    let rna = utility::io::input_from_file("data/textbook_track/rosalind_ba4a.txt")?;
    let codons = utility::io::get_codon_to_aa()?;
    println!(
        "{}",
        translate(&rna, &codons).ok_or_else(|| utility::errors::RosalindOutputError::NoneError)?
    );
    Ok(())
}
