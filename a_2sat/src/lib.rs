use std::collections::{btree_map::BTreeMap, HashMap, HashSet};

use failure::Error;

use utility;

/// 2-Satisfiability
///
/// Given: A positive integer k≤20 and k 2SAT formulas represented as follows.
/// The first line gives the number of variables n≤103 and the number of clauses m≤104,
/// each of the following m lines gives a clause of length 2 by specifying two different
/// literals: e.g., a clause (x3∨x⎯⎯⎯5) is given by 3 -5.
///
/// Return: For each formula, output 0 if it cannot be satisfied
/// or 1 followed by a satisfying assignment otherwise.
pub fn rosalind_2sat(filename: &str) -> Result<Vec<Option<Vec<isize>>>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let num_sections = lines.next().unwrap().parse::<usize>()?;
    let mut output = Vec::with_capacity(num_sections);
    for _ in 0..num_sections {
        match get_assignment(&mut lines)? {
            Some(true_variables) => {
                println!(
                    "1 {}",
                    true_variables
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                );
                output.push(Some(true_variables));
            }
            None => {
                println!("0");
                output.push(None);
            }
        }
    }
    Ok(output)
}

pub trait From2sat: Sized {
    fn from_2sat_adjacency_list(
        lines: &mut dyn Iterator<Item=String>,
        run_dfs: bool,
    ) -> Result<Self, Error>;
}
impl From2sat for utility::graph::IntegerGraph {
    fn from_2sat_adjacency_list(
        lines: &mut dyn Iterator<Item=String>,
        run_dfs: bool,
    ) -> Result<Self, Error> {
        let length_input = lines
            .next()
            .unwrap()
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<usize>, _>>()?;
        let (num_variables, num_clauses) = (length_input[0], length_input[1]);
        let mut adjacency_list = BTreeMap::new();
        let mut line;
        for _ in 0..num_clauses {
            line = lines.next().unwrap();
            let parts = line
                .split(' ')
                .map(str::parse)
                .collect::<Result<Vec<isize>, _>>()?;
            {
                let edge_list_1 = adjacency_list
                    .entry(get_node(-parts[0]))
                    .or_insert_with(Vec::new);
                edge_list_1.push(get_node(parts[1]));
                let edge_list_1 = adjacency_list
                    .entry(get_node(-parts[1]))
                    .or_insert_with(Vec::new);
                edge_list_1.push(get_node(parts[0]));
            }
        }
        Ok(Self::new(
            adjacency_list,
            (1..=num_variables * 2).collect(),
            run_dfs,
        ))
    }
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

fn get_assignment(lines: &mut dyn Iterator<Item=String>) -> Result<Option<Vec<isize>>, Error> {
    let mut graph = utility::graph::IntegerGraph::from_2sat_adjacency_list(lines, false)?;
    let graph_reverse = graph.get_reverse_graph(true);
    let mut node_order = graph_reverse
        .postvisit
        .into_iter()
        .enumerate()
        .collect::<Vec<_>>();
    node_order.sort_by(|a, b| b.1.cmp(&a.1));
    let node_order: Vec<usize> = node_order.iter().map(|(i, _)| *i).collect();
    graph.run_dfs_given_node_order(&node_order);
    let mut satisfiable = true;
    for node in (0..graph.num_nodes - 1).step_by(2) {
        if graph.connected_components[node] == graph.connected_components[node + 1] {
            satisfiable = false;
            break;
        }
    }
    if satisfiable {
        let mut component_to_nodes = HashMap::new();
        for (i, component) in graph.connected_components.iter().enumerate() {
            component_to_nodes
                .entry(component)
                .or_insert_with(Vec::new)
                .push(graph.nodes[i]);
        }
        let mut seen = HashSet::with_capacity(graph.num_nodes);
        let mut assignment = (0..graph.num_nodes).map(|_| false).collect::<Vec<_>>();
        let mut n_node;
        for node_index in node_order {
            if !seen.contains(&graph.nodes[node_index]) {
                for c_node in &component_to_nodes[&graph.connected_components[node_index]] {
                    if !seen.contains(c_node) {
                        n_node = get_negated_node(*c_node);
                        seen.insert(*c_node);
                        seen.insert(n_node);
                        assignment[graph.node_to_index[c_node]] = true;
                        assignment[graph.node_to_index[&n_node]] = false;
                    }
                }
            }
        }
        Ok(Some(
            assignment
                .into_iter()
                .enumerate()
                .filter(|(_, s)| *s)
                .map(|(i, _)| get_variable(graph.nodes[i]))
                .collect(),
        ))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use utility::io::Parseable;

    use super::*;

    #[test]
    fn _2sat() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_2sat")?;
        let result = rosalind_2sat(&input_file)?;
        for (input_assignment, output_assignment) in result.into_iter().zip(
            utility::io::input_from_file(&output_file)?
                .split('\n')
                .filter(|line| !line.trim().is_empty()),
        ) {
            if let Ok(0) = output_assignment.trim().parse::<usize>() {
                assert!(input_assignment.is_none())
            } else {
                assert!(input_assignment.is_some());
                assert_eq!(
                    input_assignment.unwrap()[..],
                    isize::parse_line(output_assignment.trim())?[1..]
                );
            }
        }
        Ok(())
    }
}
