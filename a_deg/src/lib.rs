use failure::Error;

use utility;

/// Degree Array
///
/// Given: A simple graph with n≤10^3 vertices in the edge list format.
///
/// Return: An array D[1..n] where D[i] is the degree of vertex i.
pub fn rosalind_deg(filename: &str) -> Result<Vec<usize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let graph = utility::graph::IntegerGraph::from_edge_list(&mut lines, false, false)?;
    let mut degrees = Vec::with_capacity(graph.num_nodes);
    for node in 0..graph.num_nodes {
        degrees.push(
            graph
                .adjacency_list
                .get(&graph.nodes[node])
                .unwrap_or(&Vec::new())
                .len(),
        );
    }
    println!("{}", utility::io::format_array(&degrees));
    Ok(degrees)
}

#[cfg(test)]
mod tests {
    use utility::io::Parseable;

    use super::*;

    #[test]
    fn deg() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_deg")?;
        let output = usize::parse_line(&utility::io::input_from_file(&output_file)?)?;
        assert_eq!(rosalind_deg(&input_file)?, output);
        Ok(())
    }
}
