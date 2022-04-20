use std::collections::HashMap;
use std::isize;

use anyhow::Error;
use petgraph::graph::{IndexType, NodeIndex};
use petgraph::stable_graph::StableGraph;
use petgraph::visit::EdgeRef;
use petgraph::Directed;
use petgraph::Direction::Incoming;

use std::path::Path;
use t_ba11a::get_mass_to_aa;
use t_ba5d::get_topological_ordering;
use utility::io::Parseable;

/// Sequence a Peptide
///
/// Given: A space-delimited spectral vector S.
///
/// Return: A peptide with maximum score against S. For masses with more than one amino acid, any choice may be used.
pub fn rosalind_ba11e(filename: &Path) -> Result<(), Error> {
    let mut spectrum = vec![0];
    spectrum.extend(isize::parse_line(
        &utility::io::input_from_file(filename)?
            .split('\n')
            .collect::<Vec<_>>()
            .join(" "),
    )?);
    let mut graph = StableGraph::new();
    let mut node_to_index = HashMap::new();
    for (i, value) in spectrum.iter().enumerate() {
        node_to_index.insert(i, graph.add_node(*value));
    }
    let mass_to_aa = get_mass_to_aa()?;
    for i in 0..(spectrum.len() - 1) {
        for j in (i + 1)..spectrum.len() {
            if let Some(aa) = mass_to_aa.get(&(j - i)) {
                graph.add_edge(node_to_index[&i], node_to_index[&j], aa);
            }
        }
    }
    let (_, max_path) = get_longest_path(
        &graph,
        node_to_index[&0],
        node_to_index[&(spectrum.len() - 1)],
    )
    .unwrap();
    println!(
        "{}",
        (0..(max_path.len() - 1))
            .map(|i| graph[graph.find_edge(max_path[i], max_path[i + 1]).unwrap()])
            .collect::<String>()
    );
    Ok(())
}

pub fn get_longest_path<U: Clone, Ix: IndexType>(
    graph: &StableGraph<isize, U, Directed, Ix>,
    source: NodeIndex<Ix>,
    sink: NodeIndex<Ix>,
) -> Option<(isize, Vec<NodeIndex<Ix>>)> {
    let mut weights: HashMap<_, _> = graph
        .node_indices()
        .map(|n| (n, isize::MIN + 1000))
        .collect();
    weights.insert(source, 0);
    let mut backtrack = HashMap::new();
    let topo_nodes = get_topological_ordering(&mut graph.clone());
    match topo_nodes {
        Some(topo_nodes) => {
            for node in &topo_nodes {
                if let Some((max_value, max_predecessor)) = graph
                    .edges_directed(*node, Incoming)
                    .map(|e| {
                        let (e_source, e_sink) = (e.source(), e.target());
                        (
                            graph[e_source] + graph[e_sink] + weights[&e_source],
                            e_source,
                        )
                    })
                    .max_by(|a, b| a.0.cmp(&b.0))
                {
                    weights.insert(*node, max_value);
                    backtrack.insert(*node, max_predecessor);
                }
            }
            let weight = weights[&sink];
            let mut node = sink;
            let mut longest_path = vec![node];
            while backtrack.contains_key(&node) {
                node = backtrack[&node];
                longest_path.push(node);
            }
            Some((weight, longest_path.into_iter().rev().collect()))
        }
        None => None,
    }
}
