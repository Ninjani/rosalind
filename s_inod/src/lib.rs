use anyhow::Error;

use std::path::Path;

/// Counting Phylogenetic Ancestors
///
/// Given: A positive integer n (3≤n≤10000).
///
/// Return: The number of internal nodes of any unrooted binary tree having n leaves.
pub fn rosalind_inod(filename: &Path) -> Result<usize, Error> {
    let input = utility::io::input_from_file(filename)?;
    let number = input.parse::<usize>()? - 2;
    println!("{}", number);
    Ok(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inod() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_inod")?;
        assert_eq!(
            rosalind_inod(&input_file)?,
            utility::io::input_from_file(&output_file)?.parse::<usize>()?
        );
        Ok(())
    }
}
