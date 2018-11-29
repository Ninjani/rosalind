use crate::utils;

/// Heap Sort
///
/// Given: A positive integer n≤105 and an array A[1..n] of integers from −10^5 to 10^5.
///
/// Return: A sorted array A.
pub fn rosalind_hs() {
    let (length, mut array) = utils::read_isize_array("data/algorithmic_heights/rosalind_hs.txt");
    heap_sort(&mut array, length);
    utils::print_array(&array);
}

fn heap_sort<T: Copy + PartialOrd + PartialEq>(array: &mut [T], length: usize) {
    heapify(array, length);
    for last in (1..length).rev() {
        array.swap(last, 0);
        sift_down(array, 0, last);
    }
}

fn heapify<T: Copy + PartialEq + PartialOrd>(array: &mut [T], length: usize) {
    let mid = (((length - 1) as f64) / 2.).floor() as usize;
    for i in (0..mid).rev() {
        sift_down(array, i, length);
    }
}

fn sift_down<T: PartialOrd + PartialEq + Copy>(array: &mut [T], i: usize, length: usize) {
    let mut root = i;
    let mut left;
    let mut right;
    let mut largest;
    loop {
        largest = root;
        left = 2 * root + 1;
        right = left + 1;
        if left < length && array[left] > array[largest] {
            largest = left;
        }
        if right < length && array[right] > array[largest] {
            largest = right;
        }
        if root == largest {
            return;
        } else {
            array.swap(root, largest);
            root = largest;
        }
    }
}
