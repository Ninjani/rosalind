use crate::utils;

/// Counting Phylogenetic Ancestors
///
/// Given: A positive integer n (3≤n≤10000).
///
/// Return: The number of internal nodes of any unrooted binary tree having n leaves.
pub fn rosalind_inode() {
    let num_leaves = utils::input_from_file("data/stronghold/rosalind_inod.txt")
        .parse::<usize>()
        .unwrap();
    println!("{}", num_leaves - 2);
}
