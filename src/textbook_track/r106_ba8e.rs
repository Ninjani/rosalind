use std::collections::HashMap;

use failure::Error;
use itertools::Itertools;
use petgraph::Directed;
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::Incoming;

use crate::utility;
use crate::utility::io::Parseable;

/// Implement Hierarchical Clustering
///
/// Given: An integer n, followed by an nxn distance matrix.
///
/// Return: The result of applying HierarchicalClustering to this distance matrix (using Davg),
/// with each newly created cluster listed on each line.
pub fn rosalind_ba8e(filename: &str) -> Result<Vec<Vec<usize>>, Error> {
    let contents = utility::io::input_from_file(filename)?;
    let mut lines = contents.split("\n");
    let _n = lines.next().unwrap().trim().parse::<usize>()?;
    let mut matrix = Vec::with_capacity(_n);
    for line in lines {
        matrix.push(f64::parse_line(line)?);
    }
    let (_root, _graph, clusters) = hierarchical_clustering(matrix);
    for c in &clusters {
        println!("{}", utility::io::format_array(c));
    }
    Ok(clusters)
}

fn hierarchical_clustering(
    matrix: Vec<Vec<f64>>,
) -> (
    NodeIndex<u32>,
    Graph<usize, u32, Directed, u32>,
    Vec<Vec<usize>>,
) {
    let mut remove_matrix = matrix.clone();
    fn cluster_distance_avg(cluster_1: &[usize], cluster_2: &[usize], matrix: &[Vec<f64>]) -> f64 {
        cluster_1
            .into_iter()
            .cartesian_product(cluster_2.into_iter())
            .map(|(c1, c2)| matrix[*c1][*c2])
            .sum::<f64>()
            / (cluster_1.len() * cluster_2.len()) as f64
    }
    let mut cluster_tracker = Vec::new();
    let mut clusters: Vec<Vec<usize>> = (0..matrix.len()).map(|i| vec![i]).collect();
    let mut graph = Graph::new();
    let mut index_to_node = HashMap::new();
    for i in 0..matrix.len() {
        index_to_node.insert(i, graph.add_node(i));
    }
    let mut node_index = clusters.len();
    while clusters.len() > 1 {
        let (min_c1, min_c2, _) = (0..clusters.len() - 1)
            .cartesian_product(1usize..clusters.len())
            .filter(|(c1, c2)| *c1 < *c2)
            .map(|(c1, c2)| {
                (
                    c1,
                    c2,
                    cluster_distance_avg(&clusters[c1], &clusters[c2], &matrix),
                )
            })
            .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
            .unwrap();
        let cluster_new: Vec<_> = clusters[min_c1]
            .iter()
            .chain(clusters[min_c2].iter())
            .cloned()
            .collect();
        cluster_tracker.push(cluster_new.iter().map(|i| *i + 1).collect());
        index_to_node.insert(node_index, graph.add_node(node_index));
        graph.add_edge(index_to_node[&node_index], index_to_node[&min_c1], 0);
        graph.add_edge(index_to_node[&node_index], index_to_node[&min_c2], 0);
        node_index += 1;
        remove_matrix = remove_matrix
            .into_iter()
            .enumerate()
            .filter(|(i, _)| *i != min_c1 && *i != min_c2)
            .map(|(_, row)| row)
            .collect();
        remove_matrix = (0..remove_matrix.len())
            .map(|i| {
                (0..remove_matrix[0].len())
                    .filter(|j| *j != min_c1 && *j != min_c2)
                    .map(|j| remove_matrix[i][j])
                    .collect()
            })
            .collect();
        clusters = clusters
            .into_iter()
            .enumerate()
            .filter(|(i, _)| *i != min_c1 && *i != min_c2)
            .map(|(_, c)| c)
            .collect();
        let mut new_row_col: Vec<f64> = clusters
            .iter()
            .map(|c| cluster_distance_avg(&c, &cluster_new, &matrix))
            .collect();
        remove_matrix = remove_matrix
            .into_iter()
            .enumerate()
            .map(|(j, mut row)| {
                row.push(new_row_col[j]);
                row
            })
            .collect();
        new_row_col.push(0.);
        remove_matrix.push(new_row_col);
        clusters.push(cluster_new);
    }
    let root = graph
        .node_indices()
        .filter(|node| graph.edges_directed(*node, Incoming).next().is_none())
        .next()
        .unwrap();
    (root, graph, cluster_tracker)
}
