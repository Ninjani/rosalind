use crate::algorithmic_heights::r7_mer::merge;
use crate::utils;

/// Merge Sort
///
/// Given: A positive integer n≤105 and an array A[1..n] of integers from −10^5 to 10^5.
///
/// Return: A sorted array A[1..n].
pub fn rosalind_ms() {
    let (_, array) = utils::read_isize_array("data/algorithmic_heights/rosalind_ms.txt");
    utils::print_array(&merge_sort(&array));
}

fn merge_sort<T: PartialOrd + PartialEq + Copy>(array: &[T]) -> Vec<T> {
    let length = array.len();
    if length > 1 {
        let mid = length / 2;
        merge(&merge_sort(&array[..mid]), &merge_sort(&array[mid..]))
    } else {
        array.to_vec()
    }
}
