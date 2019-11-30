use failure::Error;
use itertools::Itertools;
use petgraph::algo::astar;

use t_ba5d::read_edge_weighted_list;
use utility;

/// Compute Distances Between Leaves
///
/// Given: An integer n followed by the adjacency list of a weighted tree with n leaves.
///
/// Return: A space-separated n x n (di, j), where di, j is the length of the path between leaves i and j.
pub fn rosalind_ba7a(filename: &str) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    make_distance_matrix(&contents);
    Ok(())
}

fn make_distance_matrix(contents: &str) {
    let (graph, _) = read_edge_weighted_list(contents.split('\n').skip(1).map(|l| l.to_owned()));
    let leaves: Vec<_> = graph
        .node_indices()
        .filter(|n| graph.neighbors(*n).count() == 1)
        .sorted_by(|a, b| graph[*a].cmp(&graph[*b])).collect();
    for leaf_1 in &leaves {
        for leaf_2 in &leaves {
            print!(
                "{} ",
                astar(
                    &graph,
                    *leaf_1,
                    |finish| finish == *leaf_2,
                    |e| *e.weight(),
                    |_| 0,
                )
                    .unwrap()
                    .0
            );
        }
        println!();
    }
}
