use crate::utils;
use crate::utils::Parseable;
use std::collections::BinaryHeap;

/// Partial Sort
///
/// Given: A positive integer n≤105 and an array A[1..n] of integers from −105 to 105, a positive integer k≤1000.
///
/// Return: The k smallest elements of a sorted array A.
pub fn rosalind_ps() {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_ps.txt");
    let mut lines = contents.split('\n');
    lines.next().unwrap().parse::<usize>().unwrap();
    let array = isize::parse_line(lines.next().unwrap()).unwrap();
    let k = lines.next().unwrap().parse::<usize>().unwrap();
    utils::print_array(&partial_sort(&array, k - 1));
}

fn partial_sort(array: &[isize], k: usize) -> Vec<isize> {
    let mut heap = BinaryHeap::new();
    for (i, element) in array.iter().enumerate() {
        heap.push(*element);
        if i > k {
            heap.pop();
        }
    }
    heap.into_sorted_vec()
}
