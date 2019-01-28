use crate::utils;
use failure::Error;

/// Building a Heap
///
/// Given: A positive integer n≤10^5 and array A[1..n] of integers from −10^5 to 10^5.
///
/// Return: A permuted array A satisfying the binary max heap property: for any 2≤i≤n, A[⌊i/2⌋]≥A[i].
pub fn rosalind_hea() -> Result<(), Error> {
    let (length, mut array) = utils::read_isize_array("data/algorithmic_heights/rosalind_hea.txt")?;
    build_max_heap(&mut array, length);
    utils::print_array(&array);
    Ok(())
}

fn build_max_heap<T: PartialOrd + PartialEq + Copy>(array: &mut [T], length: usize) {
    let mid = ((length as f64) / 2.).floor() as usize;
    for i in (0..mid).rev() {
        max_heapify(array, i)
    }
}

fn max_heapify<T: PartialOrd + PartialEq + Copy>(array: &mut [T], i: usize) {
    let left = 2 * i + 1;
    let right = left + 1;
    let mut largest = i;
    let length = array.len();
    if left < length && array[left] > array[largest] {
        largest = left;
    }
    if right < length && array[right] > array[largest] {
        largest = right;
    }
    if largest != i {
        array.swap(i, largest);
        max_heapify(array, largest);
    }
}
