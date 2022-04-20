use anyhow::Error;
use itertools::Itertools;
use std::path::Path;

use utility::io::Parseable;

/// Implement FarthestFirstTraversal
///
/// Given: Integers k and m followed by a set of points Data in m-dimensional space.
///
/// Return: A set Centers consisting of k points (centers) resulting from applying
/// FarthestFirstTraversal(Data, k), where the first point from Data is chosen as the
/// first center to initialize the algorithm.

pub fn rosalind_ba8a(filename: &Path) -> Result<Vec<Vec<f64>>, Error> {
    let contents = utility::io::input_from_file(filename)?;
    let mut lines = contents.split('\n');
    let (k, _m) = lines
        .next()
        .unwrap()
        .split_whitespace()
        .collect_tuple()
        .unwrap();
    let (k, _m) = (k.parse::<usize>()?, _m.parse::<usize>()?);
    let points = lines
        .map(|line| f64::parse_line(line.trim()))
        .collect::<Result<Vec<_>, _>>()?;
    let centers = farthest_first_traversal(&points, k);
    for c in &centers {
        println!("{}", utility::io::format_array(c));
    }
    Ok(centers)
}

fn farthest_first_traversal(points: &[Vec<f64>], k: usize) -> Vec<Vec<f64>> {
    let mut centers: Vec<Vec<f64>> = Vec::with_capacity(k);
    centers.push(points[0].clone());
    while centers.len() < k {
        centers.push(
            points
                .iter()
                .map(|point| (point, center_distance(point, &centers)))
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap()
                .0
                .clone(),
        );
    }
    centers
}

pub fn center_distance(point: &[f64], centers: &[Vec<f64>]) -> f64 {
    centers
        .iter()
        .map(|c| euclidean_distance(c, point))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

pub fn euclidean_distance(vector_1: &[f64], vector_2: &[f64]) -> f64 {
    vector_1
        .iter()
        .zip(vector_2.iter())
        .map(|(v1, v2)| (*v1 - *v2).powf(2.))
        .sum::<f64>()
        .sqrt()
}
