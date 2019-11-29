use failure::Error;

use utility;

pub fn rosalind_ba2h() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba2h.txt")?;
    let lines = contents.split('\n').collect::<Vec<_>>();
    let dna: Vec<_> = lines[1].split(' ').map(|l| l.to_owned()).collect();
    println!(
        "{}",
        get_distance_between_pattern_and_strings(lines[0], &dna)
    );
    Ok(())
}

fn get_distance_between_pattern_and_strings(pattern: &str, dna: &[String]) -> usize {
    let k = pattern.len();
    let mut distance = 0;
    for text in dna {
        let mut hamming_distance = k + 1;
        for kmer in utility::string::kmerize(text, k) {
            let kmer_distance = utility::string::hamming(pattern, &kmer);
            if hamming_distance > kmer_distance {
                hamming_distance = kmer_distance;
            }
        }
        distance += hamming_distance;
    }
    distance
}
