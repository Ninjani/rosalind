use anyhow::Error;

use std::path::Path;
use utility::io::Parseable;

/// Compute the Number of Breakpoints in a Permutation
///
/// Given: A signed permutation P.
///
/// Return: The number of breakpoints in P.
pub fn rosalind_ba6b(filename: &Path) -> Result<(), Error> {
    let line = utility::io::input_from_file(filename)?;
    let mut permutation = vec![0];
    permutation.extend(isize::parse_line(&line[1..(line.len() - 1)])?);
    permutation.push(permutation.len() as isize);
    println!(
        "{}",
        (1..permutation.len())
            .filter(|i| permutation[*i] - permutation[*i - 1] != 1)
            .count()
    );
    Ok(())
}
