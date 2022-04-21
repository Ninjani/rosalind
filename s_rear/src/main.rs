use anyhow::Error;
use s_rear::rosalind_rear;
use std::path::PathBuf;

fn main() -> Result<(), Error> {
    rosalind_rear(&PathBuf::from("data/real_data/rosalind_rear.txt"))?;
    Ok(())
}
