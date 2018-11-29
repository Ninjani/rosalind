use crate::utils;
use std::collections::HashSet;
use std::iter::FromIterator;

/// Introduction to Set Operations
///
/// Given: A positive integer n (n≤20,000) and two subsets A and B of {1,2,…,n}.
///
/// Return: Six sets: A∪B, A∩B, A−B, B−A, Ac, and Bc (where set complements are taken with respect to {1,2,…,n}).
pub fn rosalind_seto() {
    let contents = utils::input_from_file("data/stronghold/rosalind_seto.txt");
    let mut lines = contents.split('\n');
    let max_n = lines.next().unwrap().parse::<usize>().unwrap();
    let set_a = utils::read_set(&lines.next().unwrap());
    let set_b = utils::read_set(&lines.next().unwrap());
    let set_u = HashSet::from_iter(1..=max_n);
    utils::print_set(&set_a.union(&set_b).collect());
    utils::print_set(&set_a.intersection(&set_b).collect());
    utils::print_set(&set_a.difference(&set_b).collect());
    utils::print_set(&set_b.difference(&set_a).collect());
    utils::print_set(&set_u.difference(&set_a).collect());
    utils::print_set(&set_u.difference(&set_b).collect());
}
