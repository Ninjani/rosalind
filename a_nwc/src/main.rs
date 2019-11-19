use failure::Error;

use crate::utility;

/// Given: A positive integer k≤20 and k simple directed graphs
/// with integer edge weights from −103 to 103 and n≤103 vertices in the edge list format.
///
/// Return: For each graph, output "1" if it contains a negative weight cycle and "-1" otherwise.
pub fn rosalind_nwc(filename: &str) -> Result<Vec<isize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let num_sections = lines.next().unwrap().parse::<usize>()?;
    let mut output = Vec::with_capacity(num_sections);
    for _ in 0..num_sections {
        let weighted_graph = utility::graph::WeightedGraph::from_weighted_edge_list(&mut lines)?;
        let mut has_negative_cycle = false;
        for node in 0..weighted_graph.num_nodes {
            if weighted_graph.bellman_ford(node).is_none() {
                has_negative_cycle = true;
                break;
            }
        }
        if has_negative_cycle {
            output.push(1);
        } else {
            output.push(-1);
        }
    }
    println!("{}", utility::io::format_array(&output));
    Ok(output)
}

#[cfg(test)]
mod tests {
    use crate::utility::io::Parseable;

    use super::*;

    #[test]
    fn nwc() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_nwc")?;
        let output = isize::parse_line(&utility::io::input_from_file(&output_file)?)?;
        assert_eq!(rosalind_nwc(&input_file)?, output);
        Ok(())
    }
}
