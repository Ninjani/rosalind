use anyhow::Error;

use s_lexf::enumerate_lex;
use std::path::Path;
use t_ba3d::de_bruijn_graph;
use t_ba3f::EulerianCycle;

pub fn rosalind_ba3i(filename: &Path) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let length = contents.parse::<usize>()?;
    let patterns = enumerate_lex(&['0', '1'], length);
    let adjacency_list = de_bruijn_graph(&patterns);
    let (index_to_node, indexed_adjacency_list) = utility::graph::convert_graph(&adjacency_list);
    let graph = utility::graph::IntegerGraph::new(
        indexed_adjacency_list,
        (0..index_to_node.len()).collect(),
        false,
    );
    let cycle = graph.get_eulerian_cycle(None).unwrap();
    let cycle_length = cycle.len();
    println!(
        "{}",
        cycle
            .into_iter()
            .take(cycle_length - length + 1)
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
