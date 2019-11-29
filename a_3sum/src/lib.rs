use failure::Error;

use crate::utility;
use crate::utility::io::Parseable;

/// 3SUM
///
/// Given: A positive integer k≤20, a postive integer n≤10^4, and k arrays of size n containing integers from −10^5 to 10^5.
///
/// Return: For each array A[1..n], output three different indices 1≤p<q<r≤n such that A[p]+A[q]+A[r]=0 if exist, and "-1" otherwise.
pub fn rosalind_3sum(filename: &str) -> Result<Vec<Option<(usize, usize, usize)>>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input.split('\n');
    let length_input = usize::parse_line(lines.next().unwrap())?;
    let length = length_input[1];
    let mut output = Vec::with_capacity(length_input[0]);
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
                println!("{}", utility::io::format_array(&real_indices));
                output.push(Some((real_indices[0], real_indices[1], real_indices[2])));
            }
            None => {
                println!("-1");
                output.push(None);
            }
        }
    }
    Ok(output)
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

#[cfg(test)]
mod tests {
    use crate::utility::io::Parseable;

    use super::*;

    #[test]
    fn _3sum() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_3sum")?;
        let result = rosalind_3sum(&input_file)?;
        let num_nones = utility::io::input_from_file(&output_file)?
            .split('\n')
            .filter(|line| line.trim() == "-1")
            .count();
        assert_eq!(result.iter().filter(|x| x.is_none()).count(), num_nones);
        for (indices, line) in result.into_iter().zip(
            utility::io::input_from_file(&input_file)?
                .split('\n')
                .skip(1),
        ) {
            let array = isize::parse_line(line)?;
            if let Some((p, q, r)) = indices {
                assert_eq!(array[p - 1] + array[q - 1] + array[r - 1], 0);
            }
        }
        Ok(())
    }
}
