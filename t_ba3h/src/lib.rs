use anyhow::Error;

use std::path::Path;
use t_ba3d::de_bruijn_graph;
use t_ba3g::EulerianPath;

pub fn rosalind_ba3h(filename: &Path) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let patterns: Vec<_> = contents.split('\n').skip(1).map(|s| s.to_owned()).collect();
    let adjacency_list = de_bruijn_graph(&patterns);
    let (index_to_node, indexed_adjacency_list) = utility::graph::convert_graph(&adjacency_list);
    let graph = utility::graph::IntegerGraph::new(
        indexed_adjacency_list,
        (0..index_to_node.len()).collect(),
        false,
    );
    println!(
        "{}",
        graph
            .get_eulerian_path()
            .unwrap()
            .into_iter()
            .enumerate()
            .map(|(i, read_index)| if i == 0 {
                index_to_node[&read_index].clone()
            } else {
                index_to_node[&read_index]
                    .chars()
                    .last()
                    .unwrap()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("")
    );
    Ok(())
}
