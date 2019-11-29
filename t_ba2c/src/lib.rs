use std::collections::HashMap;

use failure::Error;
use ndarray::Array2;

use utility;
use utility::io::Parseable;

pub fn rosalind_ba2c() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba2c.txt")?;
    let mut lines = contents.split('\n');
    let (text, k) = (
        lines.next().unwrap(),
        lines.next().unwrap().parse::<usize>()?,
    );
    let matrix = Array2::from_shape_vec(
        (4, k),
        lines
            .map(|line| f64::parse_line(line))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flat_map(|line| line.into_iter())
            .collect(),
    )?;
    println!("{}", get_profile_most_probable_kmer(&text, k, &matrix));
    Ok(())
}

pub fn get_probability_kmer(kmer: &str, profile_matrix: &Array2<f64>) -> f64 {
    let alphabet_map: HashMap<_, _> = "ACGT".chars().enumerate().map(|(n, c)| (c, n)).collect();
    kmer.chars()
        .enumerate()
        .map(|(i, c)| profile_matrix[(alphabet_map[&c], i)])
        .product()
}

pub fn get_profile_most_probable_kmer(
    text: &str,
    k: usize,
    profile_matrix: &Array2<f64>,
) -> String {
    let text: Vec<_> = text.chars().collect();
    let mut max_probability = -1.;
    let mut max_kmer = String::new();
    for i in 0..=(text.len() - k) {
        let text_kmer: String = text[i..(i + k)].iter().collect();
        let probability = get_probability_kmer(&text_kmer, profile_matrix);
        if probability > max_probability {
            max_probability = probability;
            max_kmer = text_kmer;
        }
    }
    max_kmer
}
