use crate::algorithmic_heights::r22_bf::bellman_ford;
use crate::utils;
use failure::Error;

/// Given: A positive integer k≤20 and k simple directed graphs
/// with integer edge weights from −103 to 103 and n≤103 vertices in the edge list format.
///
/// Return: For each graph, output "1" if it contains a negative weight cycle and "-1" otherwise.
pub fn rosalind_nwc() -> Result<(), Error> {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_nwc.txt");
    let mut lines = contents
        .split("\n")
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let num_sections = lines.next().unwrap().parse::<usize>()?;
    for _ in 0..num_sections {
        let (num_nodes, _, edges) = utils::read_weighted_edge_list(&mut lines)?;
        let mut has_negative_cycle = false;
        for node in 1..num_nodes + 1 {
            if bellman_ford(num_nodes, &edges, node).is_none() {
                has_negative_cycle = true;
                break;
            }
        }
        if has_negative_cycle {
            print!("1 ");
        } else {
            print!("-1 ");
        }
    }
    Ok(())
}
