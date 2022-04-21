use anyhow::Error;

use std::path::Path;

/// Strings and Lists
///
/// Given: A string s of length at most 200 letters and four integers a, b, c and d
///
/// Return: The slice of this string from indices a through b and c through d
/// (with space in between), inclusively.
/// In other words, we should include elements s[b] and s[d] in our slice.
pub fn rosalind_ini3(filename: &Path) -> Result<String, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input.split('\n');
    let string = lines.next().unwrap();
    let indices: Vec<usize> = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|x| x.parse())
        .collect::<Result<Vec<_>, _>>()?;
    let output = format!(
        "{} {}",
        &string[indices[0]..=indices[1]],
        &string[indices[2]..=indices[3]]
    );
    println!("{}", output);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ini3() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_ini3")?;
        let output = utility::io::input_from_file(&output_file)?;
        assert_eq!(rosalind_ini3(&input_file)?, output);
        Ok(())
    }
}
