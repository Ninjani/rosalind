use anyhow::Error;
use itertools::Itertools;

use std::path::Path;
use t_ba8a::euclidean_distance;
use utility::io::Parseable;

/// Implement the Lloyd Algorithm for k-Means Clustering
///
/// Given: Integers k and m followed by a set of points Data in m-dimensional space.
///
/// Return: A set Centers consisting of k points (centers) resulting from applying the Lloyd
/// algorithm to Data and Centers, where the first k points from Data are selected as the first k centers.
pub fn rosalind_ba8c(filename: &Path) -> Result<Vec<Vec<f64>>, Error> {
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
    let centers = lloyd_kmeans(&points, k);
    for c in &centers {
        println!("{}", utility::io::format_array(c));
    }
    Ok(centers)
}

fn lloyd_kmeans(points: &[Vec<f64>], k: usize) -> Vec<Vec<f64>> {
    let mut centers: Vec<Vec<_>> = points.iter().take(k).cloned().collect();
    let mut new_labels: Vec<_> = points
        .iter()
        .map(|point| {
            centers
                .iter()
                .enumerate()
                .map(|(i, c)| (i, euclidean_distance(point, c)))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap()
                .0
        })
        .collect();
    let mut old_labels = new_labels.iter().map(|i| i + 1).collect();
    while old_labels != new_labels {
        old_labels = new_labels;
        centers = (0..k)
            .map(|c| {
                let cluster_points = old_labels
                    .iter()
                    .enumerate()
                    .filter(|(_, c1)| **c1 == c)
                    .map(|(i, _)| &points[i][..])
                    .collect::<Vec<_>>();
                if cluster_points.is_empty() {
                    centers[c].clone()
                } else {
                    get_center_of_gravity(&cluster_points)
                }
            })
            .collect();
        new_labels = points
            .iter()
            .map(|point| {
                centers
                    .iter()
                    .enumerate()
                    .map(|(i, c)| (i, euclidean_distance(point, c)))
                    .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                    .unwrap()
                    .0
            })
            .collect();
    }
    centers
}

fn get_center_of_gravity(points: &[&[f64]]) -> Vec<f64> {
    let n = points.len() as f64;
    (0..points[0].len())
        .map(|i| points.iter().map(|p| p[i]).sum::<f64>() / n)
        .collect()
}
