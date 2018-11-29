use crate::utils;
use crate::stronghold::r46_edit::get_edit_distance;

pub fn rosalind_ba5g() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba5g.txt");
    let lines: Vec<_> = contents.split('\n').collect();
    println!("{}", get_edit_distance(lines[0], lines[1]))
}