use std::collections::HashSet;

use s_revc::reverse_complement;
use t_ba3b::reverse_kmerize;
use t_ba3d::de_bruijn_graph;
use t_ba3j::AllEulerian;
use utility;

/// W.I.P
pub fn rosalind_gasm(input: &str) {
    let reads: Vec<_> = input
        .split('\n')
        .flat_map(|s| vec![reverse_complement(s), s.to_owned()].into_iter())
        .collect();
    for k in 4..reads[0].len() {
        let kmers: HashSet<_> = reads
            .iter()
            .flat_map(|read| utility::string::kmerize(read, k))
            .collect();
        let kmers: Vec<_> = kmers.into_iter().collect();
        let adjacency_list = de_bruijn_graph(&kmers);
        let (index_to_node, indexed_adjacency_list) =
            utility::graph::convert_graph(&adjacency_list);
        let graph = utility::graph::IntegerGraph::new(
            indexed_adjacency_list,
            (0..index_to_node.len()).collect(),
            false,
        );
        let all_paths = graph.get_all_eulerian_cycles();
        if all_paths.len() == 2 {
            //            let length = cycle.len();
            for cycle in all_paths {
                let kmer_cycle = cycle
                    .into_iter()
                    .map(|n| index_to_node[&n].as_str())
                    .collect::<Vec<_>>();
                println!("{} {:?}", k, kmer_cycle);
                println!("{}", reverse_kmerize(&kmer_cycle));
            }
        }
    }
}
