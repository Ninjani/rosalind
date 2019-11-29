use failure::Error;

use utility;

pub fn rosalind_ba3a() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba3a.txt")?;
    let lines = contents.split('\n').collect::<Vec<_>>();
    let k = lines[0].parse::<usize>()?;
    for kmer in utility::string::kmerize(lines[1], k) {
        println!("{}", kmer);
    }
    Ok(())
}
