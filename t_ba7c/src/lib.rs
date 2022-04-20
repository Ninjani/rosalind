use std::collections::HashMap;

use anyhow::Error;
use itertools::Itertools;
use ndarray::Array2;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;

use std::path::Path;
use t_ba7b::{get_limb_length, read_matrix};

/// W.I.P

pub fn rosalind_ba7c(filename: &Path) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let mut lines = contents.split('\n');
    let num_leaves = lines.next().unwrap().parse::<usize>()?;
    let mut distance_matrix = read_matrix(&lines.collect::<Vec<_>>())?;
    let mut tree = Tree::new(num_leaves);
    additive_phylogeny(&mut tree, &mut distance_matrix, num_leaves - 1);
    for edge in tree.tree.edge_indices() {
        let (source, target) = tree.tree.edge_endpoints(edge).unwrap();
        println!(
            "{}->{}:{}",
            tree.tree[source], tree.tree[target], tree.tree[edge]
        );
        println!(
            "{}->{}:{}",
            tree.tree[target], tree.tree[source], tree.tree[edge]
        );
    }
    Ok(())
}

#[derive(Debug)]
struct Tree {
    pub tree: StableGraph<usize, usize>,
    pub node_to_index: HashMap<usize, NodeIndex<u32>>,
    pub num_leaves: usize,
    pub internal_index: usize,
}

impl Tree {
    fn new(num_leaves: usize) -> Self {
        Tree {
            tree: StableGraph::new(),
            node_to_index: HashMap::new(),
            num_leaves,
            internal_index: 0,
        }
    }

    fn add_internal_node(&mut self) -> NodeIndex<u32> {
        self.internal_index += 1;
        self.node_to_index.insert(
            self.internal_index + self.num_leaves,
            self.tree.add_node(self.internal_index + self.num_leaves),
        );
        self.node_to_index[&(self.internal_index + self.num_leaves)]
    }

    fn add_node(&mut self, index: usize) -> NodeIndex<u32> {
        match self.node_to_index.get(&index) {
            Some(node) => *node,
            None => {
                self.node_to_index.insert(index, self.tree.add_node(index));
                self.node_to_index[&index]
            }
        }
    }
}

fn additive_phylogeny(tree: &mut Tree, distance_matrix: &mut Array2<usize>, leaf_n: usize) {
    // if n = 2
    if leaf_n == 1 {
        // return the tree consisting of a single edge of length D1,2
        let (n1, n2) = (tree.add_node(0), tree.add_node(1));
        tree.tree.add_edge(n1, n2, distance_matrix[(0, 1)]);
    } else {
        //    limbLength ← Limb(D, n)
        let limb_length = get_limb_length(distance_matrix, leaf_n, leaf_n + 1);
        //    for j ← 1 to n - 1
        //        Dj,n ← Dj,n - limbLength
        //        Dn,j ← Dj,n
        for j in 0..leaf_n {
            distance_matrix[(j, leaf_n)] -= limb_length;
            distance_matrix[(leaf_n, j)] = distance_matrix[(j, leaf_n)];
        }
        //    (i,n,k) ← three leaves such that Di,k = Di,n + Dn,k
        let (leaf_i, leaf_k) = (0..leaf_n)
            .cartesian_product(0..leaf_n)
            .find(|(i, k)| {
                distance_matrix[(*i, leaf_n)] + distance_matrix[(leaf_n, *k)]
                    == distance_matrix[(*i, *k)]
            })
            .unwrap();
        let (node_i, node_k) = (tree.add_node(leaf_i), tree.add_node(leaf_k));
        //    x ← Di,n
        let (rest_i, rest_k) = (
            distance_matrix[(leaf_i, leaf_n)],
            distance_matrix[(leaf_n, leaf_k)],
        );
        //    remove row n and column n from D
        //    T ← AdditivePhylogeny(D, n - 1)
        additive_phylogeny(tree, distance_matrix, leaf_n - 1);
        //    v ← the (potentially new) node in T at distance x from i on the path between i and k
        let node_v = match (0..tree.num_leaves).find(|v| {
            distance_matrix[(leaf_i, *v)] == rest_i && distance_matrix[(*v, leaf_k)] == rest_k
        }) {
            Some(index_v) => tree.add_node(index_v),
            None => tree.add_internal_node(),
        };
        println!("{} {} {} {}", leaf_n, leaf_i, leaf_k, tree.tree[node_v]);
        tree.tree.add_edge(node_i, node_v, rest_i);
        tree.tree.add_edge(node_v, node_k, rest_k);
        if let Some(edge) = tree.tree.find_edge(node_i, node_k) {
            tree.tree.remove_edge(edge);
        }
        //    add leaf n back to T by creating a limb (v, n) of length limbLength
        let node_n = tree.add_node(leaf_n);
        tree.tree.add_edge(node_v, node_n, limb_length);
        //    return T
    }
}
