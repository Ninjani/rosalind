use anyhow::Error;
use s_edta::rosalind_edta;
use std::path::PathBuf;

fn main() -> Result<(), Error> {
    rosalind_edta(&PathBuf::from("data/real_data/rosalind_edta.txt"))?;
    Ok(())
}
