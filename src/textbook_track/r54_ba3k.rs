use crate::algorithmic_heights::{convert_graph, DFS};
use crate::textbook_track::r47_ba3d::de_bruijn_graph;
use crate::textbook_track::r50_ba3g::reverse_adjacency_list;
use crate::textbook_track::r45_ba3b::reverse_kmerize;
use crate::utils;
use failure::Error;
use std::collections::btree_map::BTreeMap;

/// W.I.P

pub fn rosalind_ba3k() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba3k.txt");
    let patterns: Vec<_> = contents.split('\n').map(|s| s.to_owned()).collect();
    let adjacency_list = de_bruijn_graph(&patterns);
    let (index_to_node, indexed_adjacency_list) = convert_graph(&adjacency_list);
    let nonbranching_paths =
        get_maximal_nonbranching_paths(&indexed_adjacency_list, index_to_node.len());
    for path in nonbranching_paths {
        let kmers = path.iter().map(|i| index_to_node[i].as_str()).collect::<Vec<_>>();
        print!("{} ", reverse_kmerize(&kmers));
    }
    Ok(())
}
// M AXIMAL N ON B RANCHING P ATHS (Graph)
//Paths
//empty list
//for each node v in Graph
//if v is not a 1-in-1-out node
//if O UT ( v ) > 0
//for each outgoing edge ( v, w ) from v
//NonBranchingPath
//the path consisting of the single edge ( v, w )
//while w is a 1-in-1-out node
//extend NonBranchingPath by the outgoing edge ( w, u ) from w
//w
//u
//add NonBranchingPath to the set Paths
//for each isolated cycle Cycle in Graph
//add Cycle to Paths
//return Paths
pub fn get_maximal_nonbranching_paths(
    adjacency_list: &BTreeMap<usize, Vec<usize>>,
    num_nodes: usize,
) -> Vec<Vec<usize>> {
    let adj_list_rev = reverse_adjacency_list(adjacency_list);
    let mut paths = Vec::new();
    fn is_one_in_one_out(node: usize, adj_list: &BTreeMap<usize, Vec<usize>>, adj_list_rev: &BTreeMap<usize, Vec<usize>>) -> bool {
        adj_list.get(&node).unwrap_or(&Vec::new()).len() == 1
            && adj_list_rev.get(&node).unwrap_or(&Vec::new()).len() == 1
    }

    for v in 0..num_nodes {
        if !is_one_in_one_out(v, adjacency_list, &adj_list_rev) {
            if let Some(outgoing_edges) = adjacency_list.get(&v) {
                for &w in outgoing_edges {
                    let mut w = w;
                    let mut path = vec![v, w];
                    while is_one_in_one_out(w, adjacency_list, &adj_list_rev) {
                        let u = adjacency_list.get(&w).unwrap()[0];
                        path.push(u);
                        w = u;
                    }
                    paths.push(path);
                }
            }
        }
    }
    fn is_isolated_cycle(nodes: &[usize], adj_list: &BTreeMap<usize, Vec<usize>>, adj_list_rev: &BTreeMap<usize, Vec<usize>>) -> bool {
        for &node in nodes {
            if !is_one_in_one_out(node, adj_list, adj_list_rev) {
                return false
            }
        }
        true
    }
    let dfs = DFS::run_dfs(adjacency_list.clone(), num_nodes);
    let topo_sort = dfs.get_topological_sort();
    for component in 0..dfs.num_connected_components {
        let component_nodes: Vec<_> = dfs
            .connected_components
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == component)
            .map(|(i, _)| i)
            .collect();
        if is_isolated_cycle(&component_nodes, adjacency_list, &adj_list_rev) {
            println!("{:?}", component_nodes);
            println!("{:?}", component_nodes.iter().map(|&i| topo_sort[i]).collect::<Vec<_>>());
        }
    }
    paths
}

