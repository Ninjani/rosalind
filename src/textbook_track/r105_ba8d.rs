use failure::Error;
use itertools::Itertools;

use crate::textbook_track::r102_ba8a::euclidean_distance;
use crate::utility;
use crate::utility::io::Parseable;

/// Implement the Soft k-Means Clustering Algorithm
///
/// Given: Integers k and m, followed by a stiffness parameter β,
/// followed by a set of points Data in m-dimensional space.
///
/// Return: A set Centers consisting of k points (centers) resulting from applying the soft k-means
/// clustering algorithm. Select the first k points from Data as the first centers for the algorithm
/// and run the algorithm for 100 steps. Results should be accurate up to three decimal places.
pub fn rosalind_ba8d(filename: &str) -> Result<Vec<Vec<f64>>, Error> {
    let contents = utility::io::input_from_file(filename)?;
    let mut lines = contents.split("\n");
    let (k, _m) = lines
        .next()
        .unwrap()
        .split_whitespace()
        .collect_tuple()
        .unwrap();
    let (k, _m) = (k.parse::<usize>()?, _m.parse::<usize>()?);
    let beta = lines.next().unwrap().trim().parse::<f64>()?;
    let points = lines
        .map(|line| f64::parse_line(line.trim()))
        .collect::<Result<Vec<_>, _>>()?;
    let centers = soft_kmeans(&points, k, beta, 100);
    for c in &centers {
        println!("{}", utility::io::format_array(c));
    }
    Ok(centers)
}

fn soft_kmeans(points: &[Vec<f64>], k: usize, beta: f64, num_iter: usize) -> Vec<Vec<f64>> {
    let m = points[0].len();
    let mut centers: Vec<Vec<_>> = points.iter().take(k).cloned().collect();
    let mut responsibilities: Vec<Vec<_>>;
    for _ in 0..num_iter {
        responsibilities = centers
            .iter()
            .map(|c| {
                points
                    .iter()
                    .map(|p| get_pull(c, p, &centers, beta))
                    .collect()
            })
            .collect();
        centers = (0..k)
            .map(|i| {
                (0..m)
                    .map(|j| {
                        responsibilities[i]
                            .iter()
                            .zip(points.iter().map(|p| p[j]))
                            .map(|(r, d)| r * d)
                            .sum::<f64>()
                            / responsibilities[i].iter().sum::<f64>()
                    })
                    .collect()
            })
            .collect();
    }
    centers
}

fn get_pull(center_i: &[f64], point_j: &[f64], centers: &[Vec<f64>], beta: f64) -> f64 {
    (-beta * euclidean_distance(point_j, center_i)).exp()
        / centers
        .into_iter()
        .map(|c_i| (-beta * euclidean_distance(point_j, c_i)).exp())
        .sum::<f64>()
}
