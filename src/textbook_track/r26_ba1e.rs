use crate::textbook_track::r23_ba1b::get_sorted_kmer_counts;
use crate::utils;
use crate::utils::Parseable;
use std::collections::HashSet;

pub fn rosalind_ba1e() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba1e.txt");
    let mut lines = contents.split('\n');
    let text = lines.next().unwrap();
    let numbers = usize::parse_line(lines.next().unwrap()).unwrap();
    let (k, l, t) = (numbers[0], numbers[1], numbers[2]);
    utils::print_array(
        &find_l_t_clumps(&text, k, l, t)
            .into_iter()
            .collect::<Vec<_>>(),
    );
}

fn find_l_t_clumps(text: &str, k: usize, l: usize, t: usize) -> HashSet<String> {
    let mut clump_kmers = HashSet::new();
    let text: Vec<_> = text.chars().collect();
    for i in 0..(text.len() - l) {
        let genome = text[i..(i + l)].iter().collect::<String>();
        let counts_tuple = get_sorted_kmer_counts(&genome, k);
        for tuple in counts_tuple {
            if tuple.1 < t {
                break;
            }
            clump_kmers.insert(tuple.0);
        }
    }
    clump_kmers
}
