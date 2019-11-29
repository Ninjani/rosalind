use failure::Error;
use itertools::Itertools;

use utility;

/// Semi-Connected Graph
///
/// Given: A positive integer k≤20 and k simple directed graphs with at most
/// 103 vertices each in the edge list format.
///
/// Return: For each graph, output "1" if the graph is semi-connected and "-1" otherwise.
pub fn rosalind_sc(filename: &str) -> Result<Vec<isize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let num_sections = lines.next().unwrap().parse::<usize>()?;
    let mut output = Vec::with_capacity(num_sections);
    for _ in 0..num_sections {
        let graph = utility::graph::IntegerGraph::from_edge_list(&mut lines, true, true)?;
        if graph.is_semi_connected() {
            output.push(1);
        } else {
            output.push(-1);
        }
    }
    println!("{}", utility::io::format_array(&output));
    Ok(output)
}

impl utility::graph::IntegerGraph {
    pub fn is_semi_connected(&self) -> bool {
        for (node_1, node_2) in self.get_topological_sort().into_iter().tuple_windows() {
            if let Some(edges) = self.adjacency_list.get(&node_1) {
                if !edges.contains(&node_2) {
                    return false;
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
    fn sc() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_sc")?;
        let output = isize::parse_line(&utility::io::input_from_file(&output_file)?)?;
        assert_eq!(rosalind_sc(&input_file)?, output);
        Ok(())
    }
}
