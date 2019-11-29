use failure::Error;

use utility;

/// Testing Acyclicity
///
/// Given: A positive integer k≤20 and k simple directed graphs in the edge list format with at most 10^3 vertices and 3⋅10^3 edges each.
///
/// Return: For each graph, output "1" if the graph is acyclic and "-1" otherwise.
pub fn rosalind_dag(filename: &str) -> Result<Vec<isize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let num_sections = lines.next().unwrap().parse::<usize>()?;
    let mut output = Vec::with_capacity(num_sections);
    for _ in 0..num_sections {
        let graph = utility::graph::IntegerGraph::from_edge_list(&mut lines, true, true)?;
        if graph.is_acyclic() {
            output.push(1);
        } else {
            output.push(-1);
        }
    }
    println!("{}", utility::io::format_array(&output));
    Ok(output)
}

impl utility::graph::IntegerGraph {
    pub fn is_acyclic(&self) -> bool {
        for node_index in 0..self.num_nodes {
            if let Some(edge_list) = self.adjacency_list.get(&self.nodes[node_index]) {
                for next_node in edge_list {
                    if self.previsit[self.node_to_index[next_node]] < self.previsit[node_index]
                        && self.previsit[node_index] < self.postvisit[node_index]
                        && self.postvisit[node_index]
                        < self.postvisit[self.node_to_index[next_node]]
                    {
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
    fn dag() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_dag")?;
        let output = isize::parse_line(&utility::io::input_from_file(&output_file)?)?;
        assert_eq!(rosalind_dag(&input_file)?, output);
        Ok(())
    }
}
