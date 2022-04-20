use anyhow::Error;
use std::path::Path;

pub fn rosalind_ba2i(filename: &Path) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let lines = contents.split('\n').collect::<Vec<_>>();
    let k = lines[0].parse::<usize>()?;
    for kmer in utility::string::kmerize(lines[1], k) {
        println!("{}", kmer);
    }
    Ok(())
}
