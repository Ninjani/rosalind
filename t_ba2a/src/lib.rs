use std::collections::HashSet;

use anyhow::Error;

use std::path::Path;
use t_ba1i::get_mismatch_sequences;
use utility::io::Parseable;

pub fn rosalind_ba1a(filename: &Path) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let mut lines = contents.split('\n');
    let numbers = usize::parse_line(lines.next().unwrap())?;
    let (k, mismatch) = (numbers[0], numbers[1]);
    let dna: Vec<_> = lines.map(|l| l.to_owned()).collect();
    println!(
        "{}",
        utility::io::format_array(
            &enumerate_motifs(&dna, k, mismatch)
                .iter()
                .collect::<Vec<_>>(),
        )
    );
    Ok(())
}

fn is_motif_in_sequence_approx(motif: &str, sequence: &str, mismatch: usize) -> bool {
    let text: Vec<_> = sequence.chars().collect();
    let k = motif.len();
    for i in 0..=(text.len() - k) {
        if utility::string::hamming(&text[i..(i + k)].iter().collect::<String>(), motif) <= mismatch
        {
            return true;
        }
    }
    false
}

fn enumerate_motifs(dna: &[String], k: usize, mismatch: usize) -> HashSet<String> {
    let mut patterns = HashSet::new();
    for text in dna {
        let text: Vec<_> = text.chars().collect();
        for i in 0..=(text.len() - k) {
            let text_kmer: String = text[i..(i + k)].iter().collect();
            for neighbor in get_mismatch_sequences(&text_kmer, mismatch) {
                if dna
                    .iter()
                    .all(|sequence| is_motif_in_sequence_approx(&neighbor, sequence, mismatch))
                {
                    patterns.insert(neighbor);
                }
            }
        }
    }
    patterns
}
