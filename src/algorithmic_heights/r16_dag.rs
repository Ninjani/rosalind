use crate::algorithmic_heights::{DFS, r5_ddeg::make_adjacency_matrix};
use crate::utils;
use hashbrown::{HashMap, HashSet};
use std::iter::repeat;

/// Testing Acyclicity
///
/// Given: A positive integer k≤20 and k simple directed graphs in the edge list format with at most 10^3 vertices and 3⋅10^3 edges each.
///
/// Return: For each graph, output "1" if the graph is acyclic and "-1" otherwise.
pub fn rosalind_dag() {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_dag.txt");
    let mut sections = contents.split("\n\n");
    sections.next().unwrap();
    for section in sections {
        let (num_nodes, _, edges) = utils::read_edge_list(section);
        let adjacency_matrix = make_adjacency_matrix(&edges, true);
        if is_acyclic(num_nodes, &adjacency_matrix) {
            print!("")
        } else {
            print!("")
        }

	let dfs = DFS::run_dfs(adjacency_matrix, num_nodes);
	if dfs.is_acyclic() {
            print!("1 ")
        } else {
            print!("-1 ")
        }
    }
}

impl DFS {
    pub fn is_acyclic(&self) -> bool {
        for node in 1..self.num_nodes+1 {
	    if let Some(edge_list) = self.adjacency_matrix.get(&node) {
            	for next_node in edge_list {
		    if self.previsit[node] < self.previsit[*next_node] && self.previsit[*next_node] < self.postvisit[node] && self.postvisit[node] < self.postvisit[*next_node] {
                        return false
                    }
                }
            }
	}
	true
    }
}

pub fn is_acyclic(num_nodes: usize, adjacency_matrix: &HashMap<usize, Vec<usize>>) -> bool {
    let mut visited = repeat(false).take(num_nodes).collect::<Vec<_>>();
    let mut visited_by_node = HashSet::new();
    for node in 1..=num_nodes {
        if is_cyclic_checker(node, &mut visited, &mut visited_by_node, adjacency_matrix) {
            return false;
        }
    }
    true
}

pub fn is_cyclic_checker<S: ::std::hash::BuildHasher>(
    node: usize,
    visited: &mut [bool],
    visited_by_node: &mut HashSet<usize, S>,
    adjacency_matrix: &HashMap<usize, Vec<usize>, S>,
) -> bool {
    if !visited[node - 1] {
        visited[node - 1] = true;
        visited_by_node.insert(node);
        if let Some(edge_list) = adjacency_matrix.get(&node) {
            for child in edge_list {
                if (!visited[*child - 1]
                    && is_cyclic_checker(*child, visited, visited_by_node, adjacency_matrix))
                    || visited_by_node.contains(child)
                {
                    return true;
                }
            }
        }
    }
    visited_by_node.remove(&node);
    false
}
