use std::collections::HashSet;

use failure::Error;
use petgraph::Directed;
use petgraph::Direction::Outgoing;
use petgraph::graph::{IndexType, NodeIndex};
use petgraph::Graph;
use petgraph::visit::EdgeRef;

use crate::stronghold::r49_nwck::parse_newick;
use crate::utility;

/// Creating a Character Table
///
/// Given: An unrooted binary tree T in Newick format for at most 200 species taxa.
///
/// Return: A character table having the same splits as the edge splits of T.
/// The columns of the character table should encode the taxa ordered lexicographically;
/// the rows of the character table may be given in any order. Also, for any given character,
/// the particular subset of taxa to which 1s are assigned is arbitrary.
pub fn rosalind_ctbl(filename: &str) -> Result<(), Error> {
    let input = utility::io::input_from_file(filename)?;
    let tree = parse_newick(&input)?;
    let mut nodes = tree
        .node_indices()
        .map(|n| (n, tree.node_weight(n).unwrap()))
        .filter(|(_, name)| !name.is_empty() && *name != ";")
        .collect::<Vec<_>>();
    nodes.sort_by(|a, b| a.1.cmp(b.1));
    let internal_nodes: Vec<_> = tree
        .node_indices()
        .filter(|n| tree.node_weight(*n).unwrap().is_empty())
        .collect();
    for internal_node in internal_nodes {
        let visitable = traverse(&tree, internal_node);
        println!(
            "{}",
            nodes
                .iter()
                .map(|(n, _)| if visitable.contains(n) { '1' } else { '0' })
                .collect::<String>()
        );
    }
    Ok(())
}

pub fn traverse<T, U, Ix: IndexType>(
    tree: &Graph<T, U, Directed, Ix>,
    start_node: NodeIndex<Ix>,
) -> HashSet<NodeIndex<Ix>> {
    let mut visited = HashSet::new();
    for edge in tree.edges_directed(start_node, Outgoing) {
        if !visited.contains(&edge.target()) {
            visited.insert(edge.target());
            visited = visited
                .union(&traverse(tree, edge.target()))
                .map(|n| n.to_owned())
                .collect();
        }
    }
    visited
}
