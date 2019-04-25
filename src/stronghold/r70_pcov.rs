use crate::textbook_track::r47_ba3d::de_bruijn_graph;
use crate::textbook_track::r49_ba3f::get_eulerian_cycle;
use crate::utils;
use crate::algorithmic_heights::convert_graph;

pub fn rosalind_pcov() {
    let contents = utils::input_from_file("data/stronghold/rosalind_pcov.txt");
    let reads: Vec<_> = contents.split('\n').map(|s| s.to_owned()).collect();
    let adjacency_list = de_bruijn_graph(&reads);
    let (index_to_node, indexed_adjacency_list) = convert_graph(&adjacency_list);
    let cycle = get_eulerian_cycle(indexed_adjacency_list, None, index_to_node.len()).unwrap();
    let length = cycle.len();
    println!(
        "{}",
        cycle
            .into_iter()
            .take(length - 1)
            .map(|n| index_to_node[&n].chars().next().unwrap())
            .collect::<String>()
    );
}
