use crate::utils;
use crate::utils::Parseable;

/// Insertion Sort
///
/// Given: A positive integer n≤10^3 and an array A[1..n] of integers.
///
/// Return: The number of swaps performed by insertion sort algorithm on A[1..n].
pub fn rosalind_ins() {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_ins.txt");
    let mut lines = contents.split('\n');
    let length = lines.next().unwrap().parse::<usize>().unwrap();
    let mut array = isize::parse_line(lines.next().unwrap()).unwrap();
    println!("{}", insertion_sort(length, &mut array));
}

fn insertion_sort<T: PartialOrd>(length: usize, array: &mut [T]) -> usize {
    let mut count = 0;
    let mut k;
    for i in 1..length {
        k = i;
        while k > 0 && array[k] < array[k - 1] {
            array.swap(k - 1, k);
            count += 1;
            k -= 1;
        }
    }
    count
}
