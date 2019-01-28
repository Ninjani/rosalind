use crate::utils;
use crate::utils::Parseable;
use failure::Error;
use std::collections::BinaryHeap;

/// Partial Sort
///
/// Given: A positive integer n≤105 and an array A[1..n] of integers from −105 to 105, a positive integer k≤1000.
///
/// Return: The k smallest elements of a sorted array A.
pub fn rosalind_ps() -> Result<(), Error> {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_ps.txt");
    let lines: Vec<_> = contents.split('\n').collect();
    let array = isize::parse_line(lines[1])?;
    let k = lines[2].parse::<usize>()?;
    utils::print_array(&partial_sort(&array, k - 1));
    Ok(())
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
