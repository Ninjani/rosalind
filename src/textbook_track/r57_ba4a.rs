use crate::stronghold::r8_prot::translate;
use crate::utils;

pub fn rosalind_ba4a() {
    let rna = utils::input_from_file("data/textbook_track/rosalind_ba4a.txt");
    println!("{}", translate(&rna).unwrap());
}
