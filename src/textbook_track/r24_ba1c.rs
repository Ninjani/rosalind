use crate::stronghold::r3_revc::reverse_complement;
use crate::utils;

pub fn rosalind_ba1c() {
    let dna = utils::input_from_file("data/textbook_track/rosalind_ba1c.txt");
    println!("{}", reverse_complement(&dna));
}
