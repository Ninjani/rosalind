use anyhow::Error;
use ndarray::{Array, Array2};

use std::path::Path;

/// Compute Limb Lengths in a Tree
///
/// Given: An integer n, followed by an integer j between 0 and n - 1,
/// followed by a space-separated additive distance matrix D (whose elements are integers).
///
/// Return: The limb length of the leaf in Tree(D) corresponding to row j of this
/// distance matrix (use 0-based indexing).
pub fn rosalind_ba7b(filename: &Path) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let mut lines = contents.split('\n');
    let num_leaves = lines.next().unwrap().parse::<usize>()?;
    let leaf_number = lines.next().unwrap().parse::<usize>()?;
    let distance_matrix = read_matrix(&lines.collect::<Vec<_>>())?;
    println!(
        "{}",
        get_limb_length(&distance_matrix, leaf_number, num_leaves)
    );
    Ok(())
}

pub fn get_limb_length(
    distance_matrix: &Array2<usize>,
    leaf_number: usize,
    num_leaves: usize,
) -> usize {
    let mut limb_length = ::std::usize::MAX;
    let mut distance;
    for i in 0..(num_leaves - 1) {
        for k in (i + 1)..num_leaves {
            if i == leaf_number || k == leaf_number {
                continue;
            }
            distance = distance_matrix[(i, leaf_number)] + distance_matrix[(leaf_number, k)]
                - distance_matrix[(i, k)];
            if distance < limb_length {
                limb_length = distance
            }
        }
    }
    limb_length / 2
}

pub fn read_matrix(lines: &[&str]) -> Result<Array2<usize>, Error> {
    let mut distance_matrix = Array2::zeros((lines.len(), lines.len()));
    for (i, line) in lines.iter().enumerate() {
        distance_matrix.row_mut(i).assign(&Array::from(
            line.split_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<usize>, _>>()?,
        ));
    }
    Ok(distance_matrix)
}
