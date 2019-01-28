use crate::utils;
use failure::Error;

/// Counting Inversions
///
/// Given: A positive integer n≤10^5 and an array A[1..n] of integers from −10^5 to 10^5.
///
/// Return: The number of inversions in A.
pub fn rosalind_inv() -> Result<(), Error> {
    let (_, array) = utils::read_isize_array("data/algorithmic_heights/rosalind_inv.txt")?;
    println!("{}", merge_sort_count(&array).1);
    Ok(())
}

fn merge_sort_count<T: PartialOrd + PartialEq + Copy>(array: &[T]) -> (Vec<T>, usize) {
    let length = array.len();
    if length > 1 {
        let mid = length / 2;
        let (left_array, count_left) = merge_sort_count(&array[..mid]);
        let (right_array, count_right) = merge_sort_count(&array[mid..]);
        let (merged_array, count_merge) = merge_count(&left_array, &right_array);
        (merged_array, count_left + count_right + count_merge)
    } else {
        (array.to_vec(), 0)
    }
}

pub fn merge_count<T: PartialOrd + PartialEq + Copy>(
    left_array: &[T],
    right_array: &[T],
) -> (Vec<T>, usize) {
    let mut i = 0;
    let mut j = 0;
    let mut inversions = 0;
    let mut sorted_array = Vec::new();
    while i < left_array.len() && j < right_array.len() {
        if left_array[i] <= right_array[j] {
            sorted_array.push(left_array[i]);
            i += 1;
        } else {
            sorted_array.push(right_array[j]);
            j += 1;
            inversions += left_array.len() - i;
        }
    }
    sorted_array.extend(&left_array[i..]);
    sorted_array.extend(&right_array[j..]);
    (sorted_array, inversions)
}
