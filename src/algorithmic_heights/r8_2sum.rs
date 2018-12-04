use crate::utils;
use crate::utils::Parseable;
use failure::Error;

/// 2SUM
///
/// Given: A positive integer k≤20, a positive integer n≤10^4, and k arrays of size n containing integers from −10^5 to 10^5.
///
/// Return: For each array A[1..n], output two different indices 1≤p<q≤n such that A[p]=−A[q] if exist, and "-1" otherwise.
pub fn rosalind_2sum() -> Result<(), Error> {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_2sum.txt");
    let mut lines = contents.split('\n');
    let length_input = usize::parse_line(lines.next().unwrap())?;
    let length = length_input[1];
    for line in lines {
        let mut array_indices = isize::parse_line(line)?
            .into_iter()
            .enumerate()
            .collect::<Vec<_>>();
        array_indices.sort_by(|a, b| a.1.cmp(&b.1));
        let array = array_indices.iter().map(|a| a.1).collect::<Vec<_>>();
        let indices = array_indices.iter().map(|a| a.0).collect::<Vec<_>>();
        match two_sum(length, &array, 0) {
            Some((index_1, index_2)) => {
                let mut real_indices = vec![indices[index_1] + 1, indices[index_2] + 1];
                real_indices.sort();
                utils::print_array(&real_indices);
            }
            None => println!("-1"),
        }
    }
    Ok(())
}

fn two_sum(length: usize, array: &[isize], target: isize) -> Option<(usize, usize)> {
    let mut index_1 = 0;
    let mut index_2 = length - 1;
    while index_1 < length {
        let sum = array[index_1] + array[index_2];
        if sum == target {
            return Some((index_1, index_2));
        }
        if sum > target {
            if index_2 >= 1 {
                index_2 -= 1;
            } else {
                break;
            }
        } else {
            index_1 += 1;
        }
        if index_1 > index_2 {
            break;
        }
    }
    None
}
