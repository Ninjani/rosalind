use crate::textbook_track::r47_ba3d::de_bruijn_graph;
use crate::textbook_track::r49_ba3f::get_eulerian_cycle;
use crate::stronghold::r3_revc::reverse_complement;
use crate::utils;

pub fn rosalind_gasm() {
    let contents = utils::input_from_file("data/stronghold/rosalind_gasm.txt");
    let reads: Vec<_> = contents.split('\n')
        .flat_map(|s| vec![reverse_complement(s), s.to_owned()].into_iter()).collect();
    let mut adjacency_list = de_bruijn_graph(&reads);
    println!("{:?}", adjacency_list);
    let cycle = get_eulerian_cycle(&mut adjacency_list);
    let length = cycle.len();
    println!(
        "{}",
        cycle
            .into_iter()
            .take(length - 1)
            .map(|n| n.chars().next().unwrap())
            .collect::<String>()
    );
}
