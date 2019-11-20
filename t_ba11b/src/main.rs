use std::collections::{HashMap, HashSet};

use failure::Error;
use petgraph::Directed;
use petgraph::Direction::Outgoing;
use petgraph::graph::{IndexType, NodeIndex};
use petgraph::Graph;
use petgraph::visit::EdgeRef;

use crate::textbook_track::r12_ba11a::{get_graph_spectrum, get_mass_to_aa};
use crate::textbook_track::r59_ba4c::get_aa_to_mass_usize;
use crate::textbook_track::r66_ba4j::get_prefix_masses;
use crate::utility;
use crate::utility::io::Parseable;

/// Given: A space-delimited list of integers, Spectrum.
///
/// Return: An amino acid string with an ideal spectrum that matches Spectrum.
pub fn rosalind_ba11b() -> Result<(), Error> {
    let mut spectrum = vec![0];
    spectrum.append(&mut usize::parse_line(&utility::io::input_from_file(
        "data/textbook_track/rosalind_ba11b.txt",
    )?)?);
    let (source, sink) = (spectrum[0], spectrum[spectrum.len() - 1]);
    let mass_to_aa = get_mass_to_aa()?;
    let aa_to_mass = get_aa_to_mass_usize()?;
    let adjacency_list = get_graph_spectrum(&spectrum, &mass_to_aa);
    let mut graph = Graph::new();
    let mut node_to_index = HashMap::new();
    let (mut index_1, mut index_2);
    for (n1, n2, aa) in adjacency_list {
        index_1 = *node_to_index
            .entry(n1)
            .or_insert_with(|| graph.add_node(n1));
        index_2 = *node_to_index
            .entry(n2)
            .or_insert_with(|| graph.add_node(n2));
        graph.add_edge(index_1, index_2, aa);
    }
    for path in all_paths(&graph, node_to_index[&source], node_to_index[&sink]) {
        let peptide = (0..(path.len() - 1))
            .map(|i| graph[graph.find_edge(path[i], path[i + 1]).unwrap()])
            .collect::<String>();
        let peptide_masses: Vec<_> = peptide.chars().map(|c| aa_to_mass[&c]).collect();
        let ideal_spectrum = get_ideal_spectrum(&peptide_masses);
        if ideal_spectrum == spectrum {
            println!("{}", peptide);
        }
    }
    Ok(())
}

pub fn get_ideal_spectrum(peptide: &[usize]) -> Vec<usize> {
    let prefix_masses = get_prefix_masses(peptide);
    let mut spectrum = Vec::with_capacity(2 * peptide.len());
    for i in 0..peptide.len() {
        spectrum.push(prefix_masses[peptide.len()] - prefix_masses[i]);
        spectrum.push(prefix_masses[i]);
    }
    spectrum.sort();
    spectrum
}

pub fn all_paths<T, U, Ix: IndexType>(
    graph: &Graph<T, U, Directed, Ix>,
    start_node: NodeIndex<Ix>,
    end_node: NodeIndex<Ix>,
) -> Vec<Vec<NodeIndex<Ix>>> {
    let mut visited = HashSet::new();
    visited.insert(start_node);
    all_paths_helper(graph, start_node, end_node, &mut visited)
}

fn all_paths_helper<T, U, Ix: IndexType>(
    graph: &Graph<T, U, Directed, Ix>,
    start_node: NodeIndex<Ix>,
    end_node: NodeIndex<Ix>,
    visited: &mut HashSet<NodeIndex<Ix>>,
) -> Vec<Vec<NodeIndex<Ix>>> {
    if start_node == end_node {
        vec![vec![end_node]]
    } else {
        let mut paths = Vec::new();
        for edge in graph.edges_directed(start_node, Outgoing) {
            let next_node = edge.target();
            if !visited.contains(&next_node) {
                visited.insert(next_node);
                let descendant_paths = all_paths_helper(graph, next_node, end_node, visited);
                visited.remove(&next_node);
                paths.extend(
                    descendant_paths
                        .into_iter()
                        .map(|path| {
                            let mut new_path = vec![start_node];
                            new_path.extend(path);
                            new_path
                        })
                        .collect::<Vec<_>>(),
                )
            }
        }
        paths
    }
}
