use crate::utils;
use failure::Error;

pub fn rosalind_ba2i() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba3a.txt");
    let lines = contents.split('\n').collect::<Vec<_>>();
    let k = lines[0].parse::<usize>()?;
    for kmer in utils::kmerize(lines[1], k) {
        println!("{}", kmer);
    }
    Ok(())
}
