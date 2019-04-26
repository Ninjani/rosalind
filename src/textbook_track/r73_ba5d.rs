use crate::utils;
use failure::Error;
use hashbrown::{HashMap, HashSet};
use petgraph::graph::{IndexType, NodeIndex};
use petgraph::stable_graph::StableGraph;
use petgraph::visit::EdgeRef;
use petgraph::Directed;
use petgraph::Direction::{Incoming, Outgoing};
use std::isize;

/// Find the Longest Path in a DAG
///
/// Given: An integer representing the source node of a graph, followed by an integer representing the sink node of the graph, followed by an edge-weighted graph. The graph is represented by a modified adjacency list in which the notation "0->1:7" indicates that an edge connects node 0 to node 1 with weight 7.
///
/// Return: The length of a longest path in the graph, followed by a longest path. (If multiple longest paths exist, you may return any one.)
pub fn rosalind_ba5d() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba5d.txt");
    let mut lines = contents.split('\n');
    let (source, sink) = (
        lines.next().unwrap().parse::<usize>()?,
        lines.next().unwrap().parse::<usize>()?,
    );
    let (mut graph, node_to_index) = read_edge_weighted_list(lines.map(|l| l.to_owned()));
    let (length, longest_path) =
        get_longest_path(&mut graph, node_to_index[&source], node_to_index[&sink]).unwrap();
    println!(
        "{}\n{}",
        length,
        longest_path
            .into_iter()
            .map(|n| graph.node_weight(n).unwrap().to_string())
            .collect::<Vec<_>>()
            .join("->")
    );
    Ok(())
}

pub fn read_edge_weighted_list(
    lines: impl Iterator<Item = String>,
) -> (
    StableGraph<usize, isize, Directed, u32>,
    HashMap<usize, NodeIndex<u32>>,
) {
    let mut graph = StableGraph::new();
    let mut node_to_index = HashMap::new();
    for line in lines {
        let parts: Vec<_> = line.split("->").collect();
        let (node_1, node_2_weight) = (parts[0].parse::<usize>().unwrap(), parts[1]);
        let parts_2: Vec<_> = node_2_weight.split(':').collect();
        let (node_2, weight) = (
            parts_2[0].parse::<usize>().unwrap(),
            parts_2[1].parse::<isize>().unwrap(),
        );
        let index_1 = *node_to_index
            .entry(node_1)
            .or_insert_with(|| graph.add_node(node_1));
        let index_2 = *node_to_index
            .entry(node_2)
            .or_insert_with(|| graph.add_node(node_2));
        graph.add_edge(index_1, index_2, weight);
    }
    (graph, node_to_index)
}

pub fn get_topological_ordering<T, U, Ix: IndexType>(
    graph: &mut StableGraph<T, U, Directed, Ix>,
) -> Option<Vec<NodeIndex<Ix>>> {
    let mut list = Vec::new();
    let mut candidates: HashSet<_> = graph
        .node_indices()
        .filter(|n| graph.edges_directed(*n, Incoming).next().is_none())
        .collect();
    let mut node;
    while !candidates.is_empty() {
        node = utils::set_pop(&mut candidates).unwrap();
        let edges: Vec<_> = graph
            .edges_directed(node, Outgoing)
            .map(|e| (e.id(), e.target()))
            .collect();
        for (edge_id, target) in edges {
            graph.remove_edge(edge_id);
            if graph.edges_directed(target, Incoming).next().is_none() {
                candidates.insert(target);
            }
        }
        list.push(node);
    }
    if graph.edge_count() > 0 {
        None
    } else {
        Some(list)
    }
}

pub fn get_longest_path<T: Clone + ::std::fmt::Debug, Ix: IndexType>(
    graph: &mut StableGraph<T, isize, Directed, Ix>,
    source: NodeIndex<Ix>,
    sink: NodeIndex<Ix>,
) -> Option<(isize, Vec<NodeIndex<Ix>>)> {
    let mut weights: HashMap<_, _> = graph.node_indices().map(|n| (n, isize::MIN)).collect();
    weights.insert(source, 0);
    let mut backtrack = HashMap::new();
    let topo_nodes = get_topological_ordering(&mut graph.clone());

    match topo_nodes {
        Some(topo_nodes) => {
            let (source_index, sink_index) = (
                topo_nodes.iter().position(|&s| s == source).unwrap(),
                topo_nodes.iter().position(|&s| s == sink).unwrap(),
            );
            for node in topo_nodes[(source_index + 1)..=sink_index].iter() {
                if let Some((max_value, max_predecessor)) = graph
                    .edges_directed(*node, Incoming)
                    .map(|e| (*e.weight() + weights[&e.source()], e.source()))
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
