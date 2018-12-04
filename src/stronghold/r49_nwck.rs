use crate::utils;
use petgraph::algo::dijkstra;
use petgraph::visit::EdgeRef;
use petgraph::Direction::Incoming;
use petgraph::{Graph, Undirected};
use failure::{Error, err_msg};
use itertools::Itertools;

/// Distances in Trees
///
/// Given: A collection of n trees (n≤40) in Newick format, with each tree containing at most 200
/// nodes; each tree Tk is followed by a pair of nodes xk and yk in Tk.
///
/// Return: A collection of n positive integers, for which the kth integer represents the distance
/// between xk and yk in Tk.
pub fn rosalind_nwck() -> Result<(), Error> {
    let trees_data = utils::input_from_file("data/stronghold/rosalind_nwck.txt");
    let trees = trees_data.split("\n\n");
    let mut path_lengths = Vec::new();
    for tree_data in trees {
        let tree_data: Vec<_> = tree_data.split('\n').collect();
        let tree = parse_newick(tree_data[0])?.into_edge_type::<Undirected>();
        let (start, end) = tree_data[1].split(' ').collect_tuple().ok_or(err_msg("NoneError"))?;
        match get_path_length(&tree, start, end) {
            Some(path_length) => path_lengths.push(path_length as usize),
            None => panic!("Start/end not found"),
        }
    }
    utils::print_array(&path_lengths);
    Ok(())
}

pub fn get_path_length(
    tree: &Graph<String, f64, Undirected>,
    start_node_name: &str,
    end_node_name: &str,
) -> Option<f64> {
    let (mut start_index, mut end_index) = (None, None);
    for index in tree.node_indices() {
        if let Some(node_id) = tree.node_weight(index) {
            if node_id == start_node_name {
                start_index = Some(index);
            }
            if node_id == end_node_name {
                end_index = Some(index);
            }
        }
    }
    match (start_index, end_index) {
        (Some(start), Some(end)) => Some(dijkstra(&tree, start, Some(end), |e| *e.weight())[&end]),
        _ => None,
    }
}

fn tokenize(text: &str, separators: &[char]) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(&separators[..]) {
        if last != index {
            tokens.push(text[last..index].to_owned());
        }
        tokens.push(matched.to_owned());
        last = index + matched.len();
    }
    if last < text.len() {
        tokens.push(text[last..].to_owned());
    }
    tokens
}

pub fn parse_newick(tree_data: &str) -> Result<Graph<String, f64>, Error> {
    let mut tree = Graph::new();
    let mut ancestors = Vec::new();
    let mut node_index = tree.add_node("".to_owned());
    let tokens = tokenize(tree_data, &[';', '(', ')', ',', ':']);
    for i in 0..tokens.len() {
        let token = tokens[i].as_str();
        match token {
            "(" => {
                // Start of a new branch
                let new_index = tree.add_node("".to_owned());
                tree.add_edge(node_index, new_index, 1.);
                ancestors.push(node_index);
                node_index = new_index;
            }
            "," => {
                // Neighbor in the same branch
                let new_index = tree.add_node("".to_owned());
                tree.add_edge(ancestors[ancestors.len() - 1], new_index, 1.);
                node_index = new_index;
            }
            ")" => {
                // Finish current branch
                node_index = ancestors.pop().ok_or(err_msg("NoneError"))?;
            }
            _ => {
                let x = tokens[i - 1].as_str();
                if x == ")" || x == "(" || x == "," {
                    // Name
                    *tree.node_weight_mut(node_index).ok_or(err_msg("NoneError"))? = token.to_owned();
                } else if x == ":" {
                    // Edge weight
                    let edge_ids: Vec<_> = tree
                        .edges_directed(node_index, Incoming)
                        .map(|edge| edge.id())
                        .collect();
                    for edge_id in edge_ids {
                        *tree.edge_weight_mut(edge_id).ok_or(err_msg("NoneError"))? = token.parse::<f64>()?;
                    }
                }
            }
        }
    }
    Ok(tree)
}
