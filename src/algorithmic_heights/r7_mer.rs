use crate::utils;
use crate::utils::Parseable;

/// Merge Two Sorted Arrays
///
/// Given: A positive integer n≤10^5 and a sorted array A[1..n] of integers from −10^5 to 10^5, a positive integer m≤105 and a sorted array B[1..m] of integers from −10^5 to 10^5.
///
/// Return: A sorted array C[1..n+m] containing all the elements of A and B.
pub fn rosalind_mer() {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_mer.txt");
    let mut lines = contents.split('\n');
    lines.next().unwrap();
    let sublist_1 = isize::parse_line(lines.next().unwrap()).unwrap();
    lines.next().unwrap();
    let sublist_2 = isize::parse_line(lines.next().unwrap()).unwrap();
    utils::print_array(&merge(&sublist_1, &sublist_2));
}

pub fn merge<T: PartialOrd + PartialEq + Copy>(left_array: &[T], right_array: &[T]) -> Vec<T> {
    let mut i = 0;
    let mut j = 0;
    let mut sorted_array = Vec::new();
    while i < left_array.len() && j < right_array.len() {
        if left_array[i] <= right_array[j] {
            sorted_array.push(left_array[i]);
            i += 1;
        } else {
            sorted_array.push(right_array[j]);
            j += 1;
        }
    }
    sorted_array.extend(&left_array[i..]);
    sorted_array.extend(&right_array[j..]);
    sorted_array
}
