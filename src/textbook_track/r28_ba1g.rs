use crate::stronghold::r6_hamm::hamming;
use crate::utils;

pub fn rosalind_ba1g() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba1g.txt");
    let lines = contents.split('\n').collect::<Vec<_>>();
    println!("{}", hamming(lines[0], lines[1]));
}
