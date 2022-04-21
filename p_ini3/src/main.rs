use anyhow::Error;
use p_ini3::rosalind_ini3;
use std::path::PathBuf;

fn main() -> Result<(), Error> {
    rosalind_ini3(&PathBuf::from("data/real_data/rosalind_ini3.txt"))?;
    Ok(())
}
