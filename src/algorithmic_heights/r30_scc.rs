use crate::algorithmic_heights::r5_ddeg::make_adjacency_matrix;
use crate::algorithmic_heights::DFS;
use crate::utils;
use failure::Error;
use hashbrown::HashMap;

pub fn rosalind_scc() -> Result<(), Error> {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_scc.txt");
    let mut lines = contents
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let (num_nodes, _, edges) = utils::read_edge_list(&mut lines);
    let adjacency_matrix = make_adjacency_matrix(&edges, true);
    let node_order = DFS::get_sink_scc_node_order(&adjacency_matrix, num_nodes);
    println!(
        "{}",
        DFS::run_dfs_given_node_order(adjacency_matrix, num_nodes, &node_order)
            .num_connected_components
    );
    Ok(())
}

impl DFS {
    pub fn get_sink_scc_node_order(
        adjacency_matrix: &HashMap<usize, Vec<usize>>,
        num_nodes: usize,
    ) -> Vec<usize> {
        let graph_reverse = DFS::run_dfs_reverse(&adjacency_matrix, num_nodes);
        let mut node_order = graph_reverse
            .postvisit
            .into_iter()
            .enumerate()
            .collect::<Vec<_>>();
        node_order.sort_by(|a, b| b.1.cmp(&a.1));
        node_order.into_iter().map(|(i, _)| i + 1).collect()
    }

    fn run_dfs_reverse(adjacency_matrix: &HashMap<usize, Vec<usize>>, num_nodes: usize) -> Self {
        let mut new_matrix = HashMap::new();
        for (node_2, edge_list) in adjacency_matrix {
            for node_1 in edge_list {
                new_matrix
                    .entry(*node_1)
                    .or_insert_with(Vec::new)
                    .push(*node_2);
            }
        }
        DFS::run_dfs(new_matrix, num_nodes)
    }

    pub fn run_dfs_given_node_order(
        adjacency_matrix: HashMap<usize, Vec<usize>>,
        num_nodes: usize,
        node_order: &[usize],
    ) -> Self {
        let mut dfs = DFS::new(adjacency_matrix, num_nodes);
        for node in node_order {
            if !dfs.visited[*node - 1] {
                dfs.explore(*node);
                dfs.num_connected_components += 1;
            }
        }
        dfs
    }
}
