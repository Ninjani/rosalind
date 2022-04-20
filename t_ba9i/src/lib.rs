use anyhow::Error;

use std::path::Path;

pub fn rosalind_ba9i(filename: &Path) -> Result<String, Error> {
    let input_string = utility::io::input_from_file(filename)?;
    let output_string = bwt(&input_string);
    println!("{}", output_string);
    Ok(output_string)
}

fn bwt(input_string: &str) -> String {
    let mut rotations = Vec::with_capacity(input_string.len());
    for i in 0..input_string.len() {
        rotations.push(format!("{}{}", &input_string[i..], &input_string[..i]));
    }
    rotations.sort();
    rotations
        .into_iter()
        .map(|r| r.chars().last().unwrap())
        .collect()
}
