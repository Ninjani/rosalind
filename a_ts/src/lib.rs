use failure::Error;

use utility;

/// Given: A simple directed acyclic graph with n≤103 vertices in the edge list format.
///
/// Return: A topological sorting (i.e., a permutation of vertices) of the graph.
pub fn rosalind_ts(filename: &str) -> Result<Vec<usize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let graph = utility::graph::IntegerGraph::from_edge_list(&mut lines, true, true)?;
    let output = graph.get_topological_sort();
    println!("{}", utility::io::format_array(&output));
    Ok(output)
}

impl utility::graph::IntegerGraph {
    pub fn get_topological_sort(&self) -> Vec<usize> {
        let mut postvisit = self.postvisit.iter().enumerate().collect::<Vec<_>>();
        postvisit.sort_by(|a, b| b.1.cmp(&a.1));
        postvisit.into_iter().map(|(i, _)| self.nodes[i]).collect()
    }
}

#[cfg(test)]
mod tests {
    use utility::io::Parseable;

    use super::*;

    #[test]
    fn ts() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_ts")?;
        let output = usize::parse_line(&utility::io::input_from_file(&output_file)?)?;
        assert_eq!(rosalind_ts(&input_file)?, output);
        Ok(())
    }
}
