use failure::Error;

use crate::utility;

/// Double-Degree Array
///
/// Given: A simple graph with n≤10^3 vertices in the edge list format.
///
/// Return: An array D[1..n] where D[i] is the sum of the degrees of i's neighbors.
pub fn rosalind_ddeg(filename: &str) -> Result<Vec<usize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let graph = utility::graph::IntegerGraph::from_edge_list(&mut lines, false, false)?;
    let degrees: Vec<_> = (0..graph.num_nodes)
        .map(|n| {
            graph
                .adjacency_list
                .get(&graph.nodes[n])
                .unwrap_or(&Vec::new())
                .len()
        })
        .collect();
    let mut degree_sums = Vec::with_capacity(graph.num_nodes);
    for node in 0..graph.num_nodes {
        match graph.adjacency_list.get(&graph.nodes[node]) {
            Some(edge_list) => degree_sums.push(
                edge_list
                    .iter()
                    .map(|n| degrees.get(graph.node_to_index[n]).unwrap_or(&0))
                    .sum::<usize>(),
            ),
            None => degree_sums.push(0),
        }
    }
    println!("{}", utility::io::format_array(&degree_sums));
    Ok(degree_sums)
}

#[cfg(test)]
mod tests {
    use crate::utility::io::Parseable;

    use super::*;

    #[test]
    fn ddeg() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_ddeg")?;
        let output = usize::parse_line(&utility::io::input_from_file(&output_file)?)?;
        assert_eq!(rosalind_ddeg(&input_file)?, output);
        Ok(())
    }
}
