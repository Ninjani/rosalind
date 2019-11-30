use std::collections::HashMap;


use failure::Error;
use itertools::Itertools;
use petgraph::Incoming;
use petgraph::stable_graph::StableGraph;

use t_ba9c::SuffixTree;
use t_ba9e::{TreeColor, NodeColor};
use utility;

pub fn rosalind_ba9p(filename: &str) -> Result<HashMap<usize, NodeColor>, Error> {
    let contents = utility::io::input_from_file(filename)?;
    let (adjacency_list, initial_node_colors_string) =
        contents.split("\n-\n").collect_tuple().unwrap();
    let mut tree = StableGraph::new();
    let mut index_to_node = HashMap::new();
    for line in adjacency_list.split('\n') {
        let (node_1, node_2s) = line.trim().split(" -> ").collect_tuple().unwrap();
        let node_1 = node_1.parse::<usize>()?;
        let node_1_node = match index_to_node.get(&Some(node_1)) {
            Some(n) => *n,
            None => {
                let n = tree.add_node(Some(node_1));
                index_to_node.insert(Some(node_1), n);
                n
            }
        };
        match node_2s {
            "{}" => (),
            node_2s => {
                for node_2 in node_2s.split(',').map(|n| n.parse::<usize>()) {
                    let node_2 = node_2?;
                    let node_2_node = match index_to_node.get(&Some(node_2)) {
                        Some(n) => *n,
                        None => {
                            let n = tree.add_node(Some(node_2));
                            index_to_node.insert(Some(node_2), n);
                            n
                        }
                    };
                    tree.add_edge(node_1_node, node_2_node, (0, 0));
                }
            }
        }
    }
    let root = tree
        .node_indices()
        .filter(|n| tree.edges_directed(*n, Incoming).next().is_none())
        .next()
        .unwrap();
    let suffix_tree = SuffixTree { root, tree };
    let mut initial_node_colors = HashMap::new();
    for line in initial_node_colors_string.split('\n') {
        let (node, color) = line.split(": ").collect_tuple().unwrap();
        let color = color.parse::<NodeColor>()?;
        let node = node.parse::<usize>()?;
        initial_node_colors.insert(index_to_node[&Some(node)], color);
    }
    for node in suffix_tree.tree.node_indices() {
        if !initial_node_colors.contains_key(&node) {
            initial_node_colors.insert(node, NodeColor::Gray);
        }
    }
    let node_colors = suffix_tree.tree_color(initial_node_colors);
    for (n, c) in &node_colors {
        println!("{}: {}", suffix_tree.tree[*n].unwrap(), c);
    }
    Ok(node_colors
        .into_iter()
        .map(|(n, c)| (suffix_tree.tree[n].unwrap(), c))
        .collect())
}


