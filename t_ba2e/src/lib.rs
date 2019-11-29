use failure::Error;

use crate::textbook_track::r39_ba2d::greedy_motif_search;
use utility;
use utility::io::Parseable;

pub fn rosalind_ba2e() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba2e.txt")?;
    let mut lines = contents.split('\n');
    let numbers = usize::parse_line(lines.next().unwrap())?;
    let (k, t) = (numbers[0], numbers[1]);
    let dna: Vec<_> = lines.map(|l| l.to_owned()).collect();
    for motif in greedy_motif_search(&dna, k, t, true) {
        println!("{}", motif);
    }
    Ok(())
}
