use std::collections::btree_map::BTreeMap;

use failure::Error;

use crate::utility;

/// Completing a Tree
///
/// Given: A positive integer n (n≤1000) and an adjacency list corresponding to a graph on n nodes
/// that contains no cycles.
///
/// Return: The minimum number of edges that can be added to the graph to produce a tree.
pub fn rosalind_tree(filename: &str) -> Result<usize, Error> {
    let input = utility::io::input_from_file(filename)?;
    let graph = utility::graph::IntegerGraph::from_weird_edge_list(&input, false, true)?;
    let number = graph.num_connected_components - 1;
    println!("{}", number);
    Ok(number)
}

impl utility::graph::IntegerGraph {
    /// Reads in an adjacency_list of the form:
    /// ```
    /// num_nodes
    /// node_1 node_2
    /// node_3 node_4
    /// ...
    /// ```
    fn from_weird_edge_list(contents: &str, directed: bool, run_dfs: bool) -> Result<Self, Error> {
        let mut lines = contents.split('\n').filter(|line| !line.trim().is_empty());
        let num_nodes = lines
            .next()
            .ok_or_else(|| {
                utility::errors::RosalindParseError::InputFormatError(String::from(
                    "Missing 'num_nodes' line",
                ))
            })?
            .parse::<usize>()?;
        let mut adjacency_list = BTreeMap::new();
        let mut min_node = ::std::usize::MAX;
        for line in lines {
            let parts = line
                .split(' ')
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?;
            let (node_1, node_2) = (
                *parts.get(0).ok_or_else(|| {
                    utility::errors::RosalindParseError::InputFormatError(String::from(
                        "Missing start_node",
                    ))
                })?,
                *parts.get(1).ok_or_else(|| {
                    utility::errors::RosalindParseError::InputFormatError(String::from(
                        "Missing end_node",
                    ))
                })?,
            );
            if node_1 < min_node {
                min_node = node_1;
            }
            if node_2 < min_node {
                min_node = node_2;
            }
            {
                let edge_list_1 = adjacency_list.entry(node_1).or_insert_with(Vec::new);
                edge_list_1.push(node_2);
            }
            if !directed {
                let edge_list_2 = adjacency_list.entry(node_2).or_insert_with(Vec::new);
                edge_list_2.push(node_1);
            }
        }
        let nodes: Vec<_> = (min_node..min_node + num_nodes).collect();
        Ok(Self::new(adjacency_list, nodes, run_dfs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_tree")?;
        assert_eq!(
            rosalind_tree(&input_file)?,
            utility::io::input_from_file(&output_file)?.parse::<usize>()?
        );
        Ok(())
    }
}
