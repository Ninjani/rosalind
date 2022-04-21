use anyhow::Error;
use p_ini4::rosalind_ini4;
use std::path::PathBuf;

fn main() -> Result<(), Error> {
    rosalind_ini4(&PathBuf::from("data/real_data/rosalind_ini4.txt"))?;
    Ok(())
}
