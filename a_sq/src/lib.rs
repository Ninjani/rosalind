use std::collections::HashSet;

use anyhow::Error;

use std::path::Path;

/// Square in a Graph
///
/// Given: A positive integer k≤20 and k simple undirected graphs with n≤400 vertices in the edge list format.
///
/// Return: For each graph, output "1" if it contains a simple cycle (that is, a cycle which doesn’t intersect itself) of length 4 and "-1" otherwise.
pub fn rosalind_sq(filename: &Path) -> Result<Vec<isize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let num_sections = lines.next().unwrap().parse::<usize>()?;
    let mut output = Vec::with_capacity(num_sections);
    for _ in 0..num_sections {
        let graph = utility::graph::IntegerGraph::from_edge_list(&mut lines, false, false)?;
        if graph.has_square() {
            output.push(1);
        } else {
            output.push(-1);
        }
    }
    println!("{}", utility::io::format_array(&output));
    Ok(output)
}

pub trait HasSquare {
    fn has_square(&self) -> bool;
}

impl HasSquare for utility::graph::IntegerGraph {
    fn has_square(&self) -> bool {
        for i in 0..self.num_nodes - 1 {
            for j in (i + 1)..self.num_nodes {
                let adj_i: HashSet<_> = self
                    .adjacency_list
                    .get(&self.nodes[i])
                    .unwrap_or(&vec![])
                    .iter()
                    .cloned()
                    .collect();
                let adj_j: HashSet<_> = self
                    .adjacency_list
                    .get(&self.nodes[j])
                    .unwrap_or(&vec![])
                    .iter()
                    .cloned()
                    .collect();
                if adj_i.intersection(&adj_j).collect::<HashSet<_>>().len() > 1 {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use utility::io::Parseable;

    use super::*;

    #[test]
    fn sq() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_sq")?;
        let output = isize::parse_line(&utility::io::input_from_file(&output_file)?)?;
        assert_eq!(rosalind_sq(&input_file)?, output);
        Ok(())
    }
}
