use crate::utils;
use crate::utils::Parseable;

/// Binary Search
///
/// Given: Two positive integers n≤10^5 and m≤10^5, a sorted array A[1..n] of integers from −10^5 to 10^5 and a list of m integers −10^5≤k_1,k_2,…,k_m≤10^5.
///
/// Return: For each k_i, output an index 1≤j≤n s.t. A[j]=k_i or "-1" if there is no such index.
pub fn rosalind_bins() {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_bins.txt");
    let mut parts = contents.split('\n');
    let _length_input = usize::parse_line(parts.next().unwrap()).unwrap();
    let array = isize::parse_line(parts.next().unwrap()).unwrap();
    let keys = isize::parse_line(parts.next().unwrap()).unwrap();
    let mut indices = Vec::new();
    for key in keys {
        match binary_search(0, &array, key) {
            Some(index) => indices.push((index + 1) as isize),
            None => indices.push(-1),
        }
    }
    utils::print_array(&indices);
}

fn binary_search<T: PartialOrd + Copy>(mid: usize, array: &[T], key: T) -> Option<usize> {
    let length = array.len();
    match length {
        1 => {
            if array[0] == key {
                Some(mid)
            } else {
                None
            }
        }
        2 => {
            if array[0] == key {
                Some(mid)
            } else if array[1] == key {
                Some(mid + 1)
            } else {
                None
            }
        }
        _ => {
            let new_mid = length / 2;
            if key < array[new_mid] {
                binary_search(mid, &array[..new_mid], key)
            } else {
                binary_search(mid + new_mid, &array[new_mid..], key)
            }
        }
    }
}
