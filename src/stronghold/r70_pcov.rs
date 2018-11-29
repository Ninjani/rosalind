use crate::textbook_track::r47_ba3d::de_bruijn_graph;
use crate::textbook_track::r49_ba3f::get_eulerian_cycle;
use crate::utils;

pub fn rosalind_pcov() {
    let contents = utils::input_from_file("data/stronghold/rosalind_pcov.txt");
    let reads: Vec<_> = contents.split('\n').map(|s| s.to_owned()).collect();
    let mut adjacency_list = de_bruijn_graph(&reads);
    let cycle = get_eulerian_cycle(&mut adjacency_list);
    let length = cycle.len();
    println!(
        "{}",
        cycle
            .into_iter()
            .take(length - 1)
            .map(|n| n.chars().nth(0).unwrap())
            .collect::<String>()
    );
}
