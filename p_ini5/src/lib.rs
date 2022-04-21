use anyhow::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::path::Path;

/// Working with Files
///
/// Given: A file containing at most 1000 lines.
///
/// Return: A file containing all the even-numbered lines from the original file. Assume 1-based numbering of lines.
pub fn rosalind_ini5(filename: &Path) -> Result<String, Error> {
    let mut output = String::new();
    for (i, line) in BufReader::new(File::open(filename)?).lines().enumerate() {
        let line = line?;
        if i % 2 == 1 {
            println!("{}", line);
            output.push_str(&line);
            output.push('\n');
        }
    }
    Ok(output.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ini5() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_ini5")?;
        let output = utility::io::input_from_file(&output_file)?;
        assert_eq!(rosalind_ini5(&input_file)?, output);
        Ok(())
    }
}
