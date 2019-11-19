use std::collections::HashSet;

use failure::Error;

use crate::textbook_track::r23_ba1b::get_sorted_kmer_counts;
use crate::utility;
use crate::utility::io::Parseable;

pub fn rosalind_ba1e() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba1e.txt")?;
    let lines = contents.split('\n').collect::<Vec<_>>();
    let numbers = usize::parse_line(lines[1])?;
    let (k, l, t) = (numbers[0], numbers[1], numbers[2]);
    println!(
        "{}",
        utility::io::format_array(
            &find_l_t_clumps(lines[0], k, l, t)
                .into_iter()
                .collect::<Vec<_>>(),
        )
    );
    Ok(())
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
