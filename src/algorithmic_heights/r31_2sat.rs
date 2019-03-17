use crate::algorithmic_heights::r5_ddeg::make_adjacency_matrix;
use crate::algorithmic_heights::DFS;
use crate::utils;
use failure::Error;
use hashbrown::{HashMap, HashSet};

fn read_2sat_adjacency_list(lines: &mut Iterator<Item=String>) -> (usize, usize, Vec<(usize, usize)>) {
    let length_input = lines
        .next()
        .unwrap()
        .split(' ')
        .map(str::parse)
        .collect::<Result<Vec<usize>, _>>()
        .unwrap();
    let (num_variables, num_clauses) = (length_input[0], length_input[1]);
    let (num_nodes, num_edges) = (num_variables * 2, num_clauses * 2);
    let mut edges = Vec::with_capacity(num_edges);
    let mut line;
    for _ in 0..num_clauses {
        line = lines.next().unwrap();
        let parts = line
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<isize>, _>>()
            .unwrap();
        edges.push((get_node(-parts[0]), get_node(parts[1])));
        edges.push((get_node(-parts[1]), get_node(parts[0])));
    }
    (num_nodes, num_edges, edges)
}

fn get_node(variable: isize) -> usize {
    if variable < 0 {
        2 * (variable.abs() as usize) - 1
    } else {
        2 * variable as usize
    }
}

fn get_variable(node: usize) -> isize {
    if node % 2 == 0 {
        (node / 2) as isize
    } else {
        -(((node + 1) / 2) as isize)
    }
}

fn get_negated_node(node: usize) -> usize {
    let variable = get_variable(node);
    if variable < 0 {
        node + 1
    } else {
        node - 1
    }
}

pub fn rosalind_2sat() -> Result<(), Error> {
    let contents = utils::input_from_file(
        "data/algorithmic_heights/rosalind_2sat.txt",
    );
    let mut lines = contents.split('\n').filter(|s| !s.trim().is_empty()).map(|s| s.to_owned());
    let num_sections = lines.next().unwrap().parse::<usize>()?;
    for _ in 0..num_sections {
        let (num_nodes, _, edges) = read_2sat_adjacency_list(&mut lines);
        let adjacency_matrix = make_adjacency_matrix(&edges, true);
        let node_order = DFS::get_sink_scc_node_order(&adjacency_matrix, num_nodes);
        let dfs_scc = DFS::run_dfs_given_node_order(adjacency_matrix, num_nodes, &node_order);
        let mut satisfiable = true;
        for i in (0..num_nodes-1).step_by(2) {
            if dfs_scc.connected_components[i] == dfs_scc.connected_components[i + 1] {
                satisfiable = false;
                break;
            }
        }
        if satisfiable {
            let mut component_to_nodes = HashMap::new();
            for (i, component) in dfs_scc.connected_components.iter().enumerate() {
                component_to_nodes
                    .entry(component)
                    .or_insert_with(Vec::new)
                    .push(i + 1);
            }
            let mut seen = HashSet::with_capacity(num_nodes);
            let mut assignment = (0..num_nodes).map(|_| false).collect::<Vec<_>>();
            let mut n_node;
            for node in node_order {
                if !seen.contains(&node) {
                    for c_node in &component_to_nodes[&dfs_scc.connected_components[node - 1]] {
                        if !seen.contains(c_node) {
                            n_node = get_negated_node(*c_node);
                            seen.insert(*c_node);
                            seen.insert(n_node);
                            assignment[*c_node - 1] = true;
                            assignment[n_node - 1] = false;
                        }
                    }
                }
            }
            let true_variables = assignment
                .into_iter()
                .enumerate()
                .filter(|(_, s)| *s)
                .map(|(i, _)| get_variable(i + 1).to_string())
                .collect::<Vec<_>>()
                .join(" ");
            println!(
                "1 {}",
                true_variables
            );
        } else {
            println!("0");
        }
    }
    Ok(())
}

