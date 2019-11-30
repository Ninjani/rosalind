use failure::Error;

use utility;

pub fn rosalind_ba3a(filename: &str) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let lines = contents.split('\n').collect::<Vec<_>>();
    let k = lines[0].parse::<usize>()?;
    for kmer in utility::string::kmerize(lines[1], k) {
        println!("{}", kmer);
    }
    Ok(())
}
