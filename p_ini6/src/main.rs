use anyhow::Error;
use p_ini6::rosalind_ini6;
use std::path::PathBuf;

fn main() -> Result<(), Error> {
    rosalind_ini6(&PathBuf::from("data/real_data/rosalind_ini6.txt"))?;
    Ok(())
}
