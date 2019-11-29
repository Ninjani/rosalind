use failure::Error;

use utility;

/// Given: A positive integer k≤20 and k simple directed acyclic graphs
/// in the edge list format with at most 103 vertices each.
///
/// Return: For each graph, if it contains a Hamiltonian path output "1"
/// followed by a Hamiltonian path (i.e., a list of vertices), otherwise output "-1".
pub fn rosalind_hdag(filename: &str) -> Result<Vec<Option<Vec<usize>>>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let num_sections = lines.next().unwrap().parse::<usize>()?;
    let mut output = Vec::with_capacity(num_sections);
    for _ in 0..num_sections {
        let graph = utility::graph::IntegerGraph::from_edge_list(&mut lines, true, true)?;
        match graph.get_hamiltonian_path() {
            None => {
                println!("-1");
                output.push(None)
            }
            Some(topo_sort) => {
                println!(
                    "1 {}",
                    topo_sort
                        .iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
                output.push(Some(topo_sort))
            }
        }
    }
    Ok(output)
}

impl utility::graph::IntegerGraph {
    pub fn get_hamiltonian_path(&self) -> Option<Vec<usize>> {
        if !self.is_acyclic() {
            None
        } else {
            let topo_sort = self.get_topological_sort();
            for i in 0..topo_sort.len() - 1 {
                match self.adjacency_list.get(&topo_sort[i]) {
                    Some(edge_list) => {
                        if !edge_list.contains(&topo_sort[i + 1]) {
                            return None;
                        }
                    }
                    None => return None,
                }
            }
            Some(topo_sort)
        }
    }
}

#[cfg(test)]
mod tests {
    use utility::io::Parseable;

    use super::*;

    #[test]
    fn hdag() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_hdag")?;
        let result = rosalind_hdag(&input_file)?;
        for (input_indices, output_line) in result
            .into_iter()
            .zip(utility::io::input_from_file(&output_file)?.split('\n'))
            {
                if output_line == "-1" {
                    assert!(input_indices.is_none());
                } else {
                    assert!(input_indices.is_some());
                    assert_eq!(
                        input_indices.unwrap(),
                        usize::parse_line(
                            &output_line
                                .split_whitespace()
                                .skip(1)
                                .collect::<Vec<_>>()
                                .join(" "),
                        )?
                    );
                }
            }
        Ok(())
    }
}
