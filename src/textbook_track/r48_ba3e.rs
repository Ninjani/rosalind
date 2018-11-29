use crate::textbook_track::r47_ba3d::de_bruijn_graph;
use crate::utils;

pub fn rosalind_ba3e() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba3e.txt");
    let patterns: Vec<_> = contents.split('\n').map(|s| s.to_owned()).collect();
    for (key, value) in de_bruijn_graph(&patterns) {
        println!("{} -> {}", key, value.join(","));
    }
}
