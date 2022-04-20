use std::collections::VecDeque;

use anyhow::Error;

use std::path::Path;

/// Testing Bipartiteness
///
/// Given: A positive integer k≤20 and k simple graphs in the edge list format with at most 10^3 vertices each.
///
/// Return: For each graph, output "1" if it is bipartite and "-1" otherwise.
pub fn rosalind_bip(filename: &Path) -> Result<Vec<isize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let num_sections = lines.next().unwrap().parse::<usize>()?;
    let mut output = Vec::with_capacity(num_sections);
    for _ in 0..num_sections {
        let graph = utility::graph::IntegerGraph::from_edge_list(&mut lines, false, false)?;
        if graph.is_bipartite() {
            output.push(1);
        } else {
            output.push(-1);
        }
    }
    println!("{}", utility::io::format_array(&output));
    Ok(output)
}

pub trait BipartiteChecker {
    fn is_bipartite(&self) -> bool;
    fn is_bipartite_checker(&self, colors: &mut [Option<bool>], node: usize) -> bool;
}

impl BipartiteChecker for utility::graph::IntegerGraph {
    fn is_bipartite(&self) -> bool {
        let mut colors = (0..self.num_nodes).map(|_| None).collect::<Vec<_>>();
        for node in 0..self.num_nodes {
            if colors[node].is_none() && !self.is_bipartite_checker(&mut colors, node) {
                return false;
            }
        }
        true
    }

    fn is_bipartite_checker(&self, colors: &mut [Option<bool>], node: usize) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(node);
        colors[node] = Some(true);
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            if let Some(edge_list) = self.adjacency_list.get(&self.nodes[node]) {
                for child in edge_list {
                    let child = self.node_to_index[child];
                    if child == node {
                        return false;
                    }
                    if colors[child].is_none() {
                        colors[child] = Some(!colors[node].unwrap());
                        queue.push_back(child);
                    } else if colors[node] == colors[child] {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use utility::io::Parseable;

    use super::*;

    #[test]
    fn bip() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_bip")?;
        let output = isize::parse_line(&utility::io::input_from_file(&output_file)?)?;
        assert_eq!(rosalind_bip(&input_file)?, output);
        Ok(())
    }
}
