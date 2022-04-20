use anyhow::Error;
use itertools::Itertools;

use std::path::Path;
use t_ba8a::center_distance;
use utility::io::Parseable;

/// Compute the Squared Error Distortion
///
/// Given: Integers k and m, followed by a set of centers Centers and a set of points Data.
///
/// Return: The squared error distortion Distortion(Data, Centers).
pub fn rosalind_ba8b(filename: &Path) -> Result<f64, Error> {
    let content = utility::io::input_from_file(filename)?;
    let mut lines = content.split('\n');
    let (k, _m) = lines
        .next()
        .unwrap()
        .split_whitespace()
        .collect_tuple()
        .unwrap();
    let (k, _m) = (k.parse::<usize>()?, _m.parse::<usize>()?);
    let mut centers = Vec::with_capacity(k);
    let mut points = Vec::new();
    for (i, line) in lines.enumerate() {
        match i.cmp(&k) {
            std::cmp::Ordering::Less => centers.push(f64::parse_line(line.trim())?),
            std::cmp::Ordering::Greater => points.push(f64::parse_line(line.trim())?),
            std::cmp::Ordering::Equal => {}
        }
    }
    let distortion = squared_error_distortion(&points, &centers);
    println!("{}", distortion);
    Ok(distortion)
}

fn squared_error_distortion(points: &[Vec<f64>], centers: &[Vec<f64>]) -> f64 {
    points
        .iter()
        .map(|point| center_distance(point, centers).powf(2.))
        .sum::<f64>()
        / points.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn ba8b() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_ba8b")?;
        let output = utility::io::input_from_file(&output_file)?.parse::<f64>()?;
        assert_approx_eq!(
            rosalind_ba8b(&input_file)?,
            output,
            utility::testing::ROSALIND_FLOAT_ERROR_F64
        );
        Ok(())
    }
}
