use crate::stronghold::r49_nwck::{get_path_length, parse_newick};
use crate::utils;
use petgraph::Undirected;
use failure::{Error, err_msg};
use itertools::Itertools;

/// Newick Format with Edge Weights
///
/// Given: A collection of n weighted trees (n≤40) in Newick format, with each tree containing
/// at most 200 nodes; each tree Tk is followed by a pair of nodes xk and yk in Tk.
///
/// Return: A collection of n numbers, for which the kth number represents the distance between xk and yk in Tk.
pub fn rosalind_nkew() -> Result<(), Error> {
    let trees_data = utils::input_from_file("data/stronghold/rosalind_nkew.txt");
    let trees = trees_data.split("\n\n");
    let mut path_lengths = Vec::new();
    for tree_data in trees {
        let tree_data: Vec<_> = tree_data.split('\n').collect();
        let tree = parse_newick(tree_data[0])?.into_edge_type::<Undirected>();
        let (start, end)= tree_data[1].split(' ').collect_tuple().ok_or(err_msg("NoneError"))?;
        match get_path_length(&tree, start, end) {
            Some(path_length) => path_lengths.push(path_length as usize),
            None => panic!("Start/end not found"),
        }
    }
    utils::print_array(&path_lengths);
    Ok(())
}
