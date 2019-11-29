use failure::Error;

use crate::textbook_track::r47_ba3d::de_bruijn_graph;
use utility;

/// Genome Assembly with Perfect Coverage
///
/// Given: A collection of (error-free) DNA k-mers (k≤50) taken from the same strand of a circular chromosome.
/// In this dataset, all k-mers from this strand of the chromosome are present,
/// and their de Bruijn graph consists of exactly one simple cycle.
///
/// Return: A cyclic superstring of minimal length containing the reads
/// (thus corresponding to a candidate cyclic chromosome).
pub fn rosalind_pcov(filename: &str) -> Result<String, Error> {
    let input = utility::io::input_from_file(filename)?;
    let reads: Vec<_> = input.split('\n').map(|s| s.to_owned()).collect();
    let adjacency_list = de_bruijn_graph(&reads);
    let (index_to_node, indexed_adjacency_list) = utility::graph::convert_graph(&adjacency_list);
    let graph = utility::graph::IntegerGraph::new(
        indexed_adjacency_list,
        (0..index_to_node.len()).collect(),
        false,
    );
    let cycle = graph.get_eulerian_cycle(None).unwrap();
    let length = cycle.len();
    let superstring = cycle
        .into_iter()
        .take(length - 1)
        .map(|n| index_to_node[&n].chars().next().unwrap())
        .collect::<String>();
    println!("{}", superstring);
    Ok(superstring)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pcov() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_pcov")?;
        assert_eq!(
            rosalind_pcov(&input_file)?,
            utility::io::input_from_file(&output_file)?
        );
        Ok(())
    }
}
