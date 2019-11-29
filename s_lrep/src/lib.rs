use std::collections::HashMap;

use failure::Error;
use petgraph::Directed;
use petgraph::Direction::{Incoming, Outgoing};
use petgraph::Graph;
use petgraph::visit::EdgeRef;

use s_ctbl::traverse;
use utility;

/// W.I.P
fn make_suffix_tree(
    lines: &[&str],
) -> Result<Graph<String, (usize, usize), Directed, usize>, Error> {
    let mut tree: Graph<String, (usize, usize), Directed, usize> = Graph::default();
    let mut node_to_index = HashMap::new();
    let (mut index_1, mut index_2);
    for line in lines {
        let parts: Vec<_> = line.split(' ').collect();
        let (node_1, node_2, start, length) = (
            parts[0],
            parts[1],
            parts[2].parse::<usize>()?,
            parts[3].parse::<usize>()?,
        );
        index_1 = *node_to_index
            .entry(node_1)
            .or_insert_with(|| tree.add_node(node_1.to_owned()));
        index_2 = *node_to_index
            .entry(node_2)
            .or_insert_with(|| tree.add_node(node_2.to_owned()));
        tree.add_edge(index_1, index_2, (start, length));
    }
    Ok(tree)
}

pub fn rosalind_lrep() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/stronghold/rosalind_lrep.txt")?;
    let lines: Vec<_> = contents.split('\n').collect();
    let (dna, k) = (lines[0], lines[1].parse::<usize>()?);
    let dna: Vec<_> = dna.chars().collect();
    let tree = make_suffix_tree(&lines[2..])?;
    let mut substring = (None, 0);
    for node in tree.node_indices() {
        let num_leaves = traverse(&tree, node)
            .into_iter()
            .filter(|n| tree.edges_directed(*n, Outgoing).count() == 0)
            .count();
        if num_leaves >= k {
            let (mut start, mut length) = (None, 0);
            let mut edges = tree.edges_directed(node, Incoming);
            while let Some(edge) = edges.next() {
                start = Some(tree[edge.id()].0);
                length += tree[edge.id()].1;
                edges = tree.edges_directed(edge.source(), Incoming);
            }
            if length > substring.1 {
                substring = (start, length);
                if let Some(start) = start {
                    println!(
                        "{} {} {}",
                        tree.node_weight(node).unwrap(),
                        num_leaves,
                        &dna[(start - 1)..(start - 1 + length)]
                            .iter()
                            .collect::<String>()
                    );
                }
            }
        }
    }
    if let (Some(start), length) = substring {
        println!(
            "{}",
            &dna[(start - 1)..(start - 1 + length)]
                .iter()
                .collect::<String>()
        );
    }
    Ok(())
}
