use std::collections::btree_map::BTreeMap;
use std::collections::HashSet;

use failure::Error;

use crate::textbook_track::r50_ba3g::reverse_adjacency_list;
use crate::utility;

pub fn rosalind_ba3m() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba3m.txt")?;
    let graph = utility::graph::IntegerGraph::from_adjacency_list(&contents, true)?;
    for path in graph.get_maximal_nonbranching_paths() {
        println!(
            "{}",
            path.into_iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join("->")
        )
    }
    Ok(())
}

impl utility::graph::IntegerGraph {
    pub fn get_maximal_nonbranching_paths(&self) -> Vec<Vec<usize>> {
        assert!(self.ran_dfs);
        let mut paths = Vec::new();
        let adjacency_list_reverse = reverse_adjacency_list(&self.adjacency_list);
        fn get_one_in_one_out(
            node: usize,
            adj_list: &BTreeMap<usize, Vec<usize>>,
            adj_list_rev: &BTreeMap<usize, Vec<usize>>,
        ) -> Option<(usize, usize)> {
            if let (Some(outgoing), Some(incoming)) = (adj_list.get(&node), adj_list_rev.get(&node))
            {
                if outgoing.len() == 1 && incoming.len() == 1 {
                    Some((incoming[0], outgoing[0]))
                } else {
                    None
                }
            } else {
                None
            }
        }
        for v in 0..self.num_nodes {
            if get_one_in_one_out(v, &self.adjacency_list, &adjacency_list_reverse).is_none() {
                if let Some(edges) = self.adjacency_list.get(&self.nodes[v]) {
                    for w in edges {
                        let mut w = *w;
                        let mut path = vec![self.nodes[v], w];
                        while let Some((_, u)) =
                        get_one_in_one_out(w, &self.adjacency_list, &adjacency_list_reverse)
                            {
                                path.push(u);
                                w = u;
                            }
                        paths.push(path);
                    }
                }
            }
        }

        let mut component_to_nodes: Vec<_> = (0..self.num_connected_components)
            .map(|_| HashSet::new())
            .collect();
        for (i, c) in self.connected_components.iter().enumerate() {
            component_to_nodes[*c].insert(i);
        }
        for use_nodes in component_to_nodes {
            let mut is_isolated_cycle = true;
            let start_node = self.nodes[*use_nodes.iter().next().unwrap()];
            let mut path = vec![start_node];
            match get_one_in_one_out(start_node, &self.adjacency_list, &adjacency_list_reverse) {
                Some((_, outgoing)) => {
                    let mut current_node = outgoing;
                    while current_node != start_node {
                        path.push(current_node);
                        match get_one_in_one_out(
                            current_node,
                            &self.adjacency_list,
                            &adjacency_list_reverse,
                        ) {
                            Some((_, outgoing)) => {
                                if use_nodes.contains(&self.node_to_index[&outgoing]) {
                                    current_node = outgoing;
                                } else {
                                    is_isolated_cycle = false;
                                    break;
                                }
                            }
                            None => {
                                is_isolated_cycle = false;
                                break;
                            }
                        }
                    }
                }
                None => is_isolated_cycle = false,
            }
            if is_isolated_cycle {
                path.push(start_node);
                paths.push(path);
            }
        }
        paths
    }
}
