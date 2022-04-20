use anyhow::Error;
use rand::{thread_rng, Rng};

use std::path::Path;
use t_ba2c::get_profile_most_probable_kmer;
use t_ba2d::{get_profile, score_motifs};
use utility::io::Parseable;

pub fn rosalind_ba2f(filename: &Path) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let mut lines = contents.split('\n');
    let numbers = usize::parse_line(lines.next().unwrap())?;
    let (k, t) = (numbers[0], numbers[1]);
    let dna: Vec<_> = lines.map(|l| l.to_owned()).collect();
    let mut best_motifs = randomized_motif_search(&dna, k, t, true);
    let mut motifs;
    for _ in 1..1000 {
        motifs = randomized_motif_search(&dna, k, t, true);
        if score_motifs(&motifs, true) < score_motifs(&best_motifs, true) {
            best_motifs = motifs
        }
    }
    for motif in best_motifs {
        println!("{}", motif);
    }
    Ok(())
}

// RANDOMIZEDMOTIFSEARCH(Dna, k, t)
//        randomly select k-mers Motifs = (Motif1, …, Motift) in each string
//            from Dna
//        BestMotifs ← Motifs
//        while forever
//            Profile ← Profile(Motifs)
//            Motifs ← Motifs(Profile, Dna)
//            if Score(Motifs) < Score(BestMotifs)
//                BestMotifs ← Motifs
//            else
//                return BestMotifs
fn randomized_motif_search(dna: &[String], k: usize, t: usize, pseudocounts: bool) -> Vec<String> {
    let mut motifs: Vec<_> = (0..t)
        .map(|i| utility::string::kmerize(&dna[i], k)[thread_rng().gen_range(0..t)].clone())
        .collect();
    let mut best_motifs = motifs.clone();
    let mut profile;
    loop {
        profile = get_profile(&motifs, pseudocounts);
        motifs = dna
            .iter()
            .map(|text| get_profile_most_probable_kmer(text, k, &profile))
            .collect();
        if score_motifs(&motifs, pseudocounts) < score_motifs(&best_motifs, pseudocounts) {
            best_motifs = motifs.clone()
        } else {
            return best_motifs;
        }
    }
}
