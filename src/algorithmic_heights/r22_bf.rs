use crate::utils;
use failure::Error;

/// Bellman-Ford Algorithm
///
/// Given: A simple directed graph with integer edge weights
/// from −10^3 to 10^3 and n≤10^3 vertices in the edge list format.
///
/// Return: An array D[1..n] where D[i] is the length of a shortest path
/// from the vertex 1 to the vertex i (D[1]=0). If i is not reachable from 1 set D[i] to x.
pub fn rosalind_bf() -> Result<(), Error> {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_bf.txt");
    let mut lines = contents.split('\n').map(|s| s.to_owned());
    let (num_nodes, _, edges) = utils::read_weighted_edge_list(&mut lines)?;
    let distances =
        bellman_ford(num_nodes, &edges, 1).ok_or_else(||format_err!("Negative cycle found"))?;
    for node in 1..=num_nodes {
        if distances[node - 1] < ::std::isize::MAX {
            print!("{} ", distances[node - 1]);
        } else {
            print!("x ");
        }
    }
    Ok(())
}

/// Finds lengths of shortest weighted (incl. negative weights) paths from start_node to each other node
/// None if a negative weight cycle exists
pub fn bellman_ford(
    num_nodes: usize,
    edges: &[(usize, usize, isize)],
    start_node: usize,
) -> Option<Vec<isize>> {
    fn update(distances: &mut [isize], edge: &(usize, usize, isize)) -> bool {
        let (node_1, node_2, weight) = edge;
        if distances[*node_1 - 1] < ::std::isize::MAX
            && distances[*node_1 - 1] + weight < distances[*node_2 - 1]
        {
            distances[*node_2 - 1] = distances[*node_1 - 1] + weight;
            true
        } else {
            false
        }
    }
    let mut distances = (0..num_nodes)
        .map(|_| ::std::isize::MAX)
        .collect::<Vec<_>>();
    distances[start_node - 1] = 0;
    let mut updated;
    for _ in 0..num_nodes - 1 {
        updated = edges.iter().any(|edge| update(&mut distances, edge));
        if !updated {
            break;
        }
    }
    let mut new_distances = distances.clone();
    for edge in edges {
        update(&mut new_distances, edge);
    }
    if (0..distances.len()).any(|i| new_distances[i] < distances[i]) {
        None
    } else {
        Some(distances)
    }
}
