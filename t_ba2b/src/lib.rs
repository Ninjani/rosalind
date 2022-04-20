use anyhow::Error;

use std::path::Path;

pub fn rosalind_ba2b(filename: &Path) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let mut lines = contents.split('\n');
    let k = lines.next().unwrap().parse::<usize>()?;
    let dna: Vec<_> = lines.map(|l| l.to_owned()).collect();
    println!("{}", minimize_hamming_in_list(&dna, k));
    Ok(())
}

fn hamming_in_text(text: &str, pattern: &str) -> usize {
    let k = pattern.len();
    let mut min_distance = k + 1;
    let text: Vec<_> = text.chars().collect();
    for i in 0..=(text.len() - k) {
        let text_kmer: String = text[i..(i + k)].iter().collect();
        let distance = utility::string::hamming(&text_kmer, pattern);
        if distance < min_distance {
            min_distance = distance
        }
    }
    min_distance
}

fn hamming_in_list(dna: &[String], pattern: &str) -> usize {
    dna.iter().map(|text| hamming_in_text(text, pattern)).sum()
}

fn minimize_hamming_in_list(dna: &[String], k: usize) -> String {
    let mut min_pattern = String::new();
    let mut min_distance = (k + 1) * dna.len();
    for text in dna {
        let text: Vec<_> = text.chars().collect();
        for i in 0..=(text.len() - k) {
            let text_kmer: String = text[i..(i + k)].iter().collect();
            let distance = hamming_in_list(dna, &text_kmer);
            if distance < min_distance {
                min_distance = distance;
                min_pattern = text_kmer;
            }
        }
    }
    min_pattern
}
