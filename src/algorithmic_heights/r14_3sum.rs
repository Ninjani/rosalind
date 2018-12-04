use crate::utils;
use crate::utils::Parseable;
use failure::Error;

/// 3SUM
///
/// Given: A positive integer k≤20, a postive integer n≤10^4, and k arrays of size n containing integers from −10^5 to 10^5.
///
/// Return: For each array A[1..n], output three different indices 1≤p<q<r≤n such that A[p]+A[q]+A[r]=0 if exist, and "-1" otherwise.
pub fn rosalind_3sum() -> Result<(), Error> {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_3sum.txt");
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
        match three_sum(length, &array, 0) {
            Some((index_1, index_2, index_3)) => {
                let mut real_indices = vec![
                    indices[index_1] + 1,
                    indices[index_2] + 1,
                    indices[index_3] + 1,
                ];
                real_indices.sort();
                utils::print_array(&real_indices);
            }
            None => println!("-1"),
        }
    }
    Ok(())
}

fn three_sum(length: usize, array: &[isize], target: isize) -> Option<(usize, usize, usize)> {
    for i in 0..=(length - 2) {
        let a = array[i];
        let (mut start, mut end) = (i + 1, length - 1);
        while start < end {
            let (b, c) = (array[start], array[end]);
            if a + b + c == target {
                return Some((i, start, end));
            } else if a + b + c > target {
                end -= 1;
            } else {
                start += 1;
            }
        }
    }
    None
}
