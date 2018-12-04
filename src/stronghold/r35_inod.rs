use crate::utils;
use failure::Error;

/// Counting Phylogenetic Ancestors
///
/// Given: A positive integer n (3≤n≤10000).
///
/// Return: The number of internal nodes of any unrooted binary tree having n leaves.
pub fn rosalind_inode() -> Result<(), Error> {
    let num_leaves = utils::input_from_file("data/stronghold/rosalind_inod.txt")
        .parse::<usize>()?;
    println!("{}", num_leaves - 2);
    Ok(())
}
