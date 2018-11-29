use crate::stronghold::r6_hamm::hamming;
use crate::utils;

pub fn rosalind_ba2h() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba2h.txt");
    let mut lines = contents.split('\n');
    let pattern = lines.next().unwrap();
    let dna: Vec<_> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|l| l.to_owned())
        .collect();
    println!(
        "{}",
        get_distance_between_pattern_and_strings(&pattern, &dna)
    );
}

fn get_distance_between_pattern_and_strings(pattern: &str, dna: &[String]) -> usize {
    let k = pattern.len();
    let mut distance = 0;
    for text in dna {
        let mut hamming_distance = k + 1;
        for kmer in utils::kmerize(text, k) {
            let kmer_distance = hamming(pattern, &kmer);
            if hamming_distance > kmer_distance {
                hamming_distance = kmer_distance;
            }
        }
        distance += hamming_distance;
    }
    distance
}
