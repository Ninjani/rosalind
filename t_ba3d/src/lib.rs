use std::collections::HashMap;

use failure::Error;

use utility;

pub fn rosalind_ba3d() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba3d.txt")?;
    let lines: Vec<_> = contents.split('\n').collect();
    let k = lines[0].parse::<usize>()?;
    let nodes = utility::string::kmerize(lines[1], k);
    for (key, value) in de_bruijn_graph(&nodes) {
        println!("{} -> {}", key, value.join(","));
    }
    Ok(())
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
