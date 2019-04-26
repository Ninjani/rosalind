use crate::algorithmic_heights::{convert_graph, DFS};
use crate::textbook_track::r45_ba3b::reverse_kmerize;
use crate::textbook_track::r50_ba3g::get_eulerian_path;
use crate::textbook_track::r50_ba3g::reverse_adjacency_list;
use crate::utils;
use failure::Error;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use std::collections::btree_map::BTreeMap;

type PairedRead = (String, String);

pub fn rosalind_ba3j() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba3j.txt");
    let (paired_reads, k, d) = read_paired_reads(&contents);
    let adjacency_list = paired_de_bruijn_graph(&paired_reads);
    let (index_to_node, indexed_adjacency_list) = convert_graph(&adjacency_list);
    let num_nodes = index_to_node.len();
    for (cycle, node_v) in get_all_eulerian_paths(indexed_adjacency_list, num_nodes) {
        if let Some(cycle) = cycle {
            println!(
                "{}",
                get_string_spelled_by_gapped_patterns(
                    &cycle
                        .into_iter()
                        .map(|n| if n == num_nodes {
                            &index_to_node[&node_v.unwrap()]
                        } else {
                            &index_to_node[&n]
                        })
                        .collect::<Vec<_>>(),
                    k,
                    d
                )
                .unwrap()
            );
        }
    }
    Ok(())
}

fn read_paired_reads(contents: &str) -> (Vec<PairedRead>, usize, usize) {
    let mut lines = contents.split('\n');
    let (k, d) = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    (
        lines
            .map(|l| l.split('|').map(|s| s.to_owned()).collect_tuple().unwrap())
            .collect(),
        k,
        d,
    )
}

pub fn paired_de_bruijn_graph(nodes: &[PairedRead]) -> HashMap<PairedRead, Vec<PairedRead>> {
    fn prefix(pr: &PairedRead) -> PairedRead {
        (
            pr.0.chars().take(pr.0.len() - 1).collect(),
            pr.1.chars().take(pr.1.len() - 1).collect(),
        )
    }
    fn suffix(pr: &PairedRead) -> PairedRead {
        (
            pr.0.chars().skip(1).collect(),
            pr.1.chars().skip(1).collect(),
        )
    }
    let mut adjacency_list = HashMap::new();
    for node in nodes {
        adjacency_list
            .entry(prefix(node))
            .or_insert_with(Vec::new)
            .push(suffix(node));
    }
    adjacency_list
}

fn get_bypass_graph(
    adjacency_list: &BTreeMap<usize, Vec<usize>>,
    incoming_u: usize,
    node_v: usize,
    outgoing_w: usize,
    num_nodes: usize,
) -> BTreeMap<usize, Vec<usize>> {
    let mut new_adj_list = BTreeMap::new();
    let new_node = num_nodes;
    for (node_1, edges) in adjacency_list {
        for node_2 in edges {
            if *node_1 == incoming_u && *node_2 == node_v {
                new_adj_list
                    .entry(*node_1)
                    .or_insert_with(Vec::new)
                    .push(new_node);
            } else if *node_1 == node_v && *node_2 == outgoing_w {
                new_adj_list
                    .entry(new_node)
                    .or_insert_with(Vec::new)
                    .push(outgoing_w);
            } else {
                new_adj_list
                    .entry(*node_1)
                    .or_insert_with(Vec::new)
                    .push(*node_2);
            }
        }
    }
    new_adj_list
}

pub fn get_all_eulerian_paths(
    adjacency_list: BTreeMap<usize, Vec<usize>>,
    num_nodes: usize,
) -> Vec<(Option<Vec<usize>>, Option<usize>)> {
    let mut graphs = HashSet::new();
    graphs.insert((
        reverse_adjacency_list(&adjacency_list),
        adjacency_list,
        None,
    ));
    while let Some((adj_list_rev, adj_list, _)) = utils::set_pop(&mut graphs) {
        let mut node_v = None;
        for (node, edges) in &adj_list_rev {
            if edges.len() > 1 {
                node_v = Some(node);
                break;
            }
        }
        match node_v {
            Some(node_v) => {
                for incoming_u in adj_list_rev.get(node_v).unwrap_or(&Vec::new()) {
                    for outgoing_w in adj_list.get(node_v).unwrap_or(&Vec::new()) {
                        let new_adj_list = get_bypass_graph(
                            &adj_list,
                            *incoming_u,
                            *node_v,
                            *outgoing_w,
                            num_nodes,
                        );
                        if DFS::run_dfs(new_adj_list.clone(), num_nodes + 1)
                            .visited
                            .into_iter()
                            .all(|x| x)
                        {
                            graphs.insert((
                                reverse_adjacency_list(&new_adj_list),
                                new_adj_list,
                                Some(*node_v),
                            ));
                        }
                    }
                }
            }
            None => {
                graphs.insert((adj_list_rev, adj_list, None));
                break;
            }
        }
    }
    graphs
        .into_iter()
        .map(|(_, adj_list, node_v)| {
            (
                get_eulerian_path(
                    adj_list,
                    if node_v.is_some() {
                        num_nodes + 1
                    } else {
                        num_nodes
                    },
                ),
                node_v,
            )
        })
        .collect()
}

fn get_string_spelled_by_gapped_patterns(
    gapped_patterns: &[&PairedRead],
    k: usize,
    d: usize,
) -> Option<String> {
    let first_patterns = gapped_patterns
        .iter()
        .map(|(k, _)| k.as_str())
        .collect::<Vec<_>>();
    let second_patterns = gapped_patterns
        .iter()
        .map(|(_, k)| k.as_str())
        .collect::<Vec<_>>();
    let mut prefix_string: Vec<_> = reverse_kmerize(&first_patterns).chars().collect();
    let suffix_string: Vec<_> = reverse_kmerize(&second_patterns).chars().collect();
    for i in (k + d + 1)..prefix_string.len() {
        if prefix_string[i] != suffix_string[i - k - d] {
            return None;
        }
    }
    let suffix_length = suffix_string.len();
    prefix_string.extend(suffix_string.into_iter().skip(suffix_length - k - d));
    Some(prefix_string.into_iter().collect())
}
