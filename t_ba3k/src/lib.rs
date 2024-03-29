use anyhow::Error;

use std::path::Path;
use t_ba3b::reverse_kmerize;
use t_ba3d::de_bruijn_graph;
use t_ba3m::MaximalNonbranching;

pub fn rosalind_ba3k(filename: &Path) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let kmers: Vec<_> = contents.split('\n').map(|s| s.to_owned()).collect();
    let adjacency_list = de_bruijn_graph(&kmers);
    let (index_to_node, indexed_adjacency_list) = utility::graph::convert_graph(&adjacency_list);
    let graph = utility::graph::IntegerGraph::new(
        indexed_adjacency_list,
        (0..index_to_node.len()).collect(),
        true,
    );
    for path in graph.get_maximal_nonbranching_paths() {
        let mut path = path;
        let length = path.len();
        if path[0] == path[length - 1] {
            path = path.into_iter().take(length - 1).collect();
        }
        let kmers_path: Vec<_> = path
            .into_iter()
            .map(|i| index_to_node[&i].as_str())
            .collect();
        print!("{} ", reverse_kmerize(&kmers_path));
    }
    Ok(())
}
