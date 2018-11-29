use crate::utils;
use crate::utils::Parseable;
use rand::{thread_rng, Rng};

/// Median
///
/// Given: A positive integer n≤105 and an array A[1..n] of integers from −10^5 to 10^5, a positive number k≤n.
///
/// Return: The k-th smallest element of A.
pub fn rosalind_med() {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_med.txt");
    let mut lines = contents.split('\n');
    let length = lines.next().unwrap().parse::<usize>().unwrap();
    let mut array = isize::parse_line(lines.next().unwrap()).unwrap();
    let k = lines.next().unwrap().parse::<usize>().unwrap();
    println!("{}", select(&mut array, 0, length - 1, k - 1))
}

fn partition(array: &mut [isize], left: usize, right: usize, pivot_index: usize) -> usize {
    let pivot = array[pivot_index];
    array.swap(pivot_index, right);
    let mut store_index = left;
    for i in left..right {
        if array[i] < pivot {
            array.swap(store_index, i);
            store_index += 1;
        }
    }
    array.swap(right, store_index);
    store_index
}

fn select(array: &mut [isize], left: usize, right: usize, k: usize) -> isize {
    let mut pivot_index;
    let (mut left, mut right) = (left, right);
    loop {
        if left == right {
            return array[left];
        }
        pivot_index = thread_rng().gen_range(left, right);
        pivot_index = partition(array, left, right, pivot_index);
        if pivot_index == k {
            return array[k];
        } else if pivot_index < k {
            left = pivot_index + 1;
        } else {
            right = pivot_index - 1;
        }
    }
}
