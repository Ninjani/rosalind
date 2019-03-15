use crate::utils;
use hashbrown::HashMap;

pub fn get_degrees<T: Eq + Clone + ::std::hash::Hash>(edges: &[(T, T)]) -> HashMap<T, usize> {
    let mut degrees = HashMap::new();
    for (node_1, node_2) in edges {
        {
            let degree_1 = degrees.entry(node_1.clone()).or_insert(0);
            *degree_1 += 1;
        }
        {
            let degree_2 = degrees.entry(node_2.clone()).or_insert(0);
            *degree_2 += 1;
        }
    }
    degrees
}

/// Degree Array
///
/// Given: A simple graph with n≤10^3 vertices in the edge list format.
///
/// Return: An array D[1..n] where D[i] is the degree of vertex i.
pub fn rosalind_deg() {
    let (num_nodes, _, edges) = utils::read_edge_list(&utils::input_from_file(
        "data/algorithmic_heights/rosalind_deg.txt",
    ));
    let degrees = get_degrees(&edges);
    for node in 1..=num_nodes {
        print!("{}", degrees.get(&node).unwrap_or(&0));
    }
}
