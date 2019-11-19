use std::collections::{HashMap, HashSet};
use std::collections::VecDeque;

use failure::Error;

use crate::utility;

/// Breadth-First Search
///
/// Given: A simple directed graph with n≤10^3 vertices in the edge list format.
///
/// Return: An array D[1..n] where D[i] is the length of a shortest path from the vertex 1 to the vertex i (D[1]=0).
/// If i is not reachable from 1 set D[i] to −1.
pub fn rosalind_bfs(filename: &str) -> Result<Vec<isize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let graph = utility::graph::IntegerGraph::from_edge_list(&mut lines, true, false)?;
    let mut lengths = Vec::with_capacity(graph.num_nodes);
    for node in 0..graph.num_nodes {
        lengths.push(graph.bfs_length(0, node).map(|x| x as isize).unwrap_or(-1));
    }
    println!("{}", utility::io::format_array(&lengths));
    Ok(lengths)
}

impl utility::graph::IntegerGraph {
    fn get_path_length(node: usize, path: &HashMap<usize, Option<usize>>) -> usize {
        let mut node = node;
        let mut length = 0;
        while let Some(n) = path[&node] {
            node = n;
            length += 1;
        }
        length
    }

    /// Finds length of shortest path from start_node to end_node using breadth-first search
    fn bfs_length(&self, start_node: usize, end_node: usize) -> Option<usize> {
        let mut open_set = VecDeque::new();
        let mut closed_set = HashSet::new();
        open_set.push_back(start_node);
        let mut path = HashMap::new();
        path.insert(start_node, None);
        while !open_set.is_empty() {
            let subtree_root = open_set.pop_front().unwrap();
            if subtree_root == end_node {
                return Some(Self::get_path_length(subtree_root, &path));
            }
            if let Some(edge_list) = self.adjacency_list.get(&self.nodes[subtree_root]) {
                for child in edge_list {
                    let child = self.node_to_index[child];
                    if !closed_set.contains(&child) && !open_set.contains(&child) {
                        path.insert(child, Some(subtree_root));
                        open_set.push_back(child);
                    }
                }
            }
            closed_set.insert(subtree_root);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::utility::io::Parseable;

    use super::*;

    #[test]
    fn bfs() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_bfs")?;
        let output = isize::parse_line(&utility::io::input_from_file(&output_file)?)?;
        assert_eq!(rosalind_bfs(&input_file)?, output);
        Ok(())
    }
}
