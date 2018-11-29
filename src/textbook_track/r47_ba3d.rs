use crate::utils;
use std::collections::HashMap;

pub fn rosalind_ba3d() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba3d.txt");
    let lines: Vec<_> = contents.split('\n').collect();
    let k = lines[0].parse::<usize>().unwrap();
    let text = lines[1];
    let nodes = utils::kmerize(text, k);
    for (key, value) in de_bruijn_graph(&nodes) {
        println!("{} -> {}", key, value.join(","));
    }
}

pub fn de_bruijn_graph(nodes: &[String]) -> HashMap<String, Vec<String>> {
    let mut adjacency_list = HashMap::new();
    for node in nodes {
        let (node_l, node_r) = (
            node.chars().take(node.len() - 1).collect::<String>(),
            node.chars().skip(1).collect::<String>(),
        );
        adjacency_list
            .entry(node_l)
            .or_insert_with(Vec::new)
            .push(node_r);
    }
    adjacency_list
}
