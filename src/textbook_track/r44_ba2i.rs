use crate::utils;

pub fn rosalind_ba2i() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba3a.txt");
    let mut lines = contents.split('\n');
    let k = lines.next().unwrap().parse::<usize>().unwrap();
    let text = lines.next().unwrap();
    for kmer in utils::kmerize(&text, k) {
        println!("{}", kmer);
    }
}
