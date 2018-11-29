use crate::utils;
use crate::utils::Parseable;
use ndarray::{Array, Array2};

/// Find the Length of a Longest Path in a Manhattan-like Grid
///
/// Given: Integers n and m, followed by an n × (m+1) matrix Down and an (n+1) × m matrix Right.
/// The two matrices are separated by the "-" symbol.
///
/// Return: The length of a longest path from source (0, 0) to sink (n, m) in the n × m rectangular
/// grid whose edges are defined by the matrices Down and Right.
pub fn rosalind_ba5b() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba5b.txt");
    let mut parts = contents.split("\n-\n");
    let mut size_down = parts.next().unwrap().split('\n');
    let size = usize::parse_line(size_down.next().unwrap()).unwrap();
    let mut down = Array2::zeros((size[0], size[1] + 1));
    for (i, line) in size_down.enumerate() {
        down.row_mut(i)
            .assign(&Array::from_vec(usize::parse_line(line).unwrap()));
    }
    let mut right = Array2::zeros((size[0] + 1, size[1]));
    for (i, line) in parts.next().unwrap().split('\n').enumerate() {
        right
            .row_mut(i)
            .assign(&Array::from_vec(usize::parse_line(line).unwrap()));
    }
    println!("{}", longest_path_length(&down, &right));
}

fn longest_path_length(down: &Array2<usize>, right: &Array2<usize>) -> usize {
    let (n, m) = (down.shape()[0], right.shape()[1]);
    let mut lengths = Array2::<usize>::zeros((n + 1, m + 1));
    for i in 1..=n {
        lengths[(i, 0)] = lengths[(i - 1, 0)] + down[(i - 1, 0)];
    }
    for j in 1..=m {
        lengths[(0, j)] = lengths[(0, j - 1)] + right[(0, j - 1)];
    }
    for i in 1..=n {
        for j in 1..=m {
            lengths[(i, j)] = (lengths[(i - 1, j)] + down[(i - 1, j)])
                .max(lengths[(i, j - 1)] + right[(i, j - 1)]);
        }
    }
    lengths[(n, m)]
}
