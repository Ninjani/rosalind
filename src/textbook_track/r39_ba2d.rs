use crate::stronghold::r6_hamm::hamming;
use crate::textbook_track::r38_ba2c::get_profile_most_probable_kmer;
use crate::utils;
use crate::utils::Parseable;
use itertools::Itertools;
use ndarray::{Array1, Array2};
use hashbrown::HashMap;
use failure::Error;

pub fn rosalind_ba2d() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba2d.txt");
    let mut lines = contents.split('\n');
    let numbers = usize::parse_line(lines.next().unwrap())?;
    let (k, t) = (numbers[0], numbers[1]);
    let dna: Vec<_> = lines.map(|l| l.to_owned()).collect();
    for motif in greedy_motif_search(&dna, k, t, false) {
        println!("{}", motif);
    }
    Ok(())
}

pub fn get_profile(sequences: &[String], pseudocounts: bool) -> Array2<f64> {
    let sequence_length = sequences[0].len();
    let count_matrix = get_count_matrix(sequences, pseudocounts);
    let mut matrix = Array2::zeros((4, sequence_length));
    for i in 0..sequence_length {
        let column = count_matrix.column(i).to_vec();
        let total: usize = column.iter().sum();
        matrix.column_mut(i).assign(&Array1::from_iter(
            column.into_iter().map(|x| (x as f64) / (total as f64)),
        ));
    }
    matrix
}

fn get_count_matrix(sequences: &[String], pseudocounts: bool) -> Array2<usize> {
    let sequence_length = sequences[0].len();
    let mut matrix = Array2::zeros((4, sequence_length));
    for i in 0..sequence_length {
        let position_string = sequences
            .iter()
            .map(|sequence| sequence.chars().nth(i).unwrap())
            .collect::<String>();
        let counter = utils::char_counter(&position_string);
        matrix
            .column_mut(i)
            .assign(&Array1::from_iter("ACGT".chars().map(|c| {
                let count = *counter.get(&c).unwrap_or(&0);
                if pseudocounts {
                    count + 1
                } else {
                    count
                }
            })));
    }
    matrix
}

fn get_consensus(count_matrix: &Array2<usize>) -> String {
    let mut consensus = String::with_capacity(count_matrix.shape()[1]);
    let alphabet_map: HashMap<_, _> = "ACGT".chars().enumerate().collect();
    for i in 0..count_matrix.shape()[1] {
        let sorted_column = count_matrix
            .column(i)
            .into_iter()
            .enumerate()
            .sorted_by(|a, b| a.1.cmp(b.1).reverse());
        consensus.push(alphabet_map[&sorted_column[0].0]);
    }
    consensus
}

pub fn score_motifs(motifs: &[String], pseudocounts: bool) -> usize {
    let consensus = get_consensus(&get_count_matrix(motifs, pseudocounts));
    motifs.iter().map(|motif| hamming(motif, &consensus)).sum()
}

pub fn greedy_motif_search(dna: &[String], k: usize, t: usize, pseudocounts: bool) -> Vec<String> {
    let mut best_motifs: Vec<_> = dna
        .iter()
        .map(|text| {
            text.chars().collect::<Vec<_>>()[..k]
                .iter()
                .collect::<String>()
        })
        .collect();
    let text_1: Vec<_> = dna[0].chars().collect();
    let mut profile;
    for m in 0..=(text_1.len() - k) {
        let mut motifs = vec![text_1[m..(m + k)].iter().collect::<String>()];
        for i in 1..t {
            profile = get_profile(&motifs[..i], pseudocounts);
            motifs.push(get_profile_most_probable_kmer(&dna[i], k, &profile));
        }
        if score_motifs(&motifs, pseudocounts) < score_motifs(&best_motifs, pseudocounts) {
            best_motifs = motifs;
        }
    }
    best_motifs
}
