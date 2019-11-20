use failure::Error;

use crate::stronghold::r3_revc::reverse_complement;
use crate::utility;

pub fn rosalind_ba1c() -> Result<(), Error> {
    let dna = utility::io::input_from_file("data/textbook_track/rosalind_ba1c.txt")?;
    println!("{}", reverse_complement(&dna));
    Ok(())
}
