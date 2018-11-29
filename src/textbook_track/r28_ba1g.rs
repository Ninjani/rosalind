use crate::stronghold::r6_hamm::hamming;
use crate::utils;

pub fn rosalind_ba1g() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba1g.txt");
    let mut lines = contents.split('\n');
    let (dna_1, dna_2) = (lines.next().unwrap(), lines.next().unwrap());
    println!("{}", hamming(&dna_1, &dna_2));
}
