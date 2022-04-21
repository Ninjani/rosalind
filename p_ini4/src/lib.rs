use anyhow::Error;

use std::path::Path;

/// Conditions and Loops
///
/// Given: Two positive integers a and b (a<b<10000).
///
/// Return: The sum of all odd integers from a through b, inclusively.
pub fn rosalind_ini4(filename: &Path) -> Result<usize, Error> {
    let input: Vec<usize> = utility::io::input_from_file(filename)?
        .trim()
        .split(' ')
        .map(|x| x.parse())
        .collect::<Result<Vec<_>, _>>()?;
    let (a, b) = (input[0], input[1]);
    let mut sum = 0;
    for i in a..=b {
        if i % 2 == 1 {
            sum += i;
        }
    }
    println!("{}", sum);
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ini4() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_ini4")?;
        let output = utility::io::input_from_file(&output_file)?.parse()?;
        assert_eq!(rosalind_ini4(&input_file)?, output);
        Ok(())
    }
}
