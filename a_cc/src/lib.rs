use failure::Error;

use utility;

/// Connected Components
///
/// Given: A simple graph with n≤10^3 vertices in the edge list format.
///
/// Return: The number of connected components in the graph.
pub fn rosalind_cc(filename: &str) -> Result<usize, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let graph = utility::graph::IntegerGraph::from_edge_list(&mut lines, false, true)?;
    println!("{}", graph.num_connected_components);
    Ok(graph.num_connected_components)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cc() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_cc")?;
        let output = utility::io::input_from_file(&output_file)?.parse::<usize>()?;
        assert_eq!(rosalind_cc(&input_file)?, output);
        Ok(())
    }
}
