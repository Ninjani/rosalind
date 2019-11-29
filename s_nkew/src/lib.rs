use failure::Error;
use itertools::Itertools;
use petgraph::Undirected;

use crate::stronghold::r49_nwck::{get_path_length, parse_newick};
use utility;

/// Newick Format with Edge Weights
///
/// Given: A collection of n weighted trees (n≤40) in Newick format, with each tree containing
/// at most 200 nodes; each tree Tk is followed by a pair of nodes xk and yk in Tk.
///
/// Return: A collection of n numbers, for which the kth number represents the distance between xk and yk in Tk.
pub fn rosalind_nkew(filename: &str) -> Result<Vec<usize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let trees = input.split("\n\n");
    let mut path_lengths = Vec::new();
    for tree_data in trees {
        let tree_data: Vec<_> = tree_data.split('\n').collect();
        let tree = parse_newick(tree_data[0])?.into_edge_type::<Undirected>();
        let (start, end) = tree_data[1]
            .split(' ')
            .collect_tuple()
            .ok_or_else(|| utility::errors::RosalindOutputError::NoneError)?;
        match get_path_length(&tree, start, end) {
            Some(path_length) => path_lengths.push(path_length as usize),
            None => panic!("Start/end not found"),
        }
    }
    println!("{}", utility::io::format_array(&path_lengths));
    Ok(path_lengths)
}

#[cfg(test)]
mod tests {
    use utility::io::Parseable;

    use super::*;

    #[test]
    fn nkew() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_nkew")?;
        assert_eq!(
            rosalind_nkew(&input_file)?,
            usize::parse_line(&utility::io::input_from_file(&output_file)?)?
        );
        Ok(())
    }
}
