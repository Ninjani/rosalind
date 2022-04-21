use anyhow::Error;
use p_ini5::rosalind_ini5;
use std::path::PathBuf;

fn main() -> Result<(), Error> {
    rosalind_ini5(&PathBuf::from("data/real_data/rosalind_ini5.txt"))?;
    Ok(())
}
