use crate::textbook_track::r38_ba2c::get_probability_kmer;
use crate::textbook_track::r39_ba2d::{get_profile, score_motifs};
use crate::utils;
use crate::utils::Parseable;
use ndarray::Array2;
use rand::{thread_rng, Rng};
use random_choice::random_choice;

pub fn rosalind_ba2g() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba2g.txt");
    let mut lines = contents.split('\n');
    let numbers = usize::parse_line(lines.next().unwrap()).unwrap();
    let (k, t, n) = (numbers[0], numbers[1], numbers[2]);
    let dna: Vec<_> = lines.map(|l| l.to_owned()).collect();
    let mut best_motifs = gibbs_sampler(&dna, k, t, n, true);
    let mut motifs;
    for _ in 1..20 {
        motifs = gibbs_sampler(&dna, k, t, n, true);
        if score_motifs(&motifs, true) < score_motifs(&best_motifs, true) {
            best_motifs = motifs
        }
    }
    for motif in best_motifs {
        println!("{}", motif);
    }
}

fn get_profile_random_kmer(text: &str, k: usize, profile_matrix: &Array2<f64>) -> String {
    let kmers = utils::kmerize(text, k);
    let probabilities: Vec<_> = kmers
        .iter()
        .map(|kmer| get_probability_kmer(kmer, profile_matrix))
        .collect();
    random_choice().random_choice_f64(&kmers, &probabilities, 1)[0].to_owned()
}

// GIBBSSAMPLER(Dna, k, t, N)
//        randomly select k-mers Motifs = (Motif1, …, Motift) in each string
//            from Dna
//        BestMotifs ← Motifs
//        for j ← 1 to N
//            i ← Random(t)
//            Profile ← profile matrix constructed from all strings in Motifs
//                       except for Motifi
//            Motifi ← Profile-randomly generated k-mer in the i-th sequence
//            if Score(Motifs) < Score(BestMotifs)
//                BestMotifs ← Motifs
//        return BestMotifs
fn gibbs_sampler(dna: &[String], k: usize, t: usize, n: usize, pseudocounts: bool) -> Vec<String> {
    let mut motifs: Vec<_> = (0..t)
        .map(|i| utils::kmerize(&dna[i], k)[thread_rng().gen_range(0, t)].clone())
        .collect();
    let mut best_motifs = motifs.clone();
    let mut profile;
    let mut i;
    for _ in 0..n {
        i = thread_rng().gen_range(0, t);
        profile = get_profile(&[&motifs[..i], &motifs[(i + 1)..]].concat(), pseudocounts);
        motifs[i] = get_profile_random_kmer(&dna[i], k, &profile);
        if score_motifs(&motifs, pseudocounts) < score_motifs(&best_motifs, pseudocounts) {
            best_motifs = motifs.clone()
        }
    }
    best_motifs
}
