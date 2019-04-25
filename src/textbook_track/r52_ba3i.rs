use crate::algorithmic_heights::convert_graph;
use crate::stronghold::r23_lexf::enumerate_lex_2;
use crate::textbook_track::r47_ba3d::de_bruijn_graph;
use crate::textbook_track::r49_ba3f::get_eulerian_cycle;
use crate::utils;
use failure::Error;

pub fn rosalind_ba3i() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba3i.txt");
    let length = contents.parse::<usize>()?;
    let patterns = enumerate_lex_2(&['0', '1'], length);
    let adjacency_list = de_bruijn_graph(&patterns);
    let (index_to_node, indexed_adjacency_list) = convert_graph(&adjacency_list);
    let cycle = get_eulerian_cycle(indexed_adjacency_list, None, index_to_node.len()).unwrap();
    let cycle_length = cycle.len();
    println!(
        "{}",
        cycle
            .into_iter()
            .take(cycle_length - length + 1)
            .enumerate()
            .map(|(i, read_index)| if i == 0 {
                index_to_node[&read_index].clone()
            } else {
                index_to_node[&read_index]
                    .chars()
                    .last()
                    .unwrap()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("")
    );
    Ok(())
}
