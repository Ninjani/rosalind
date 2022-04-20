use anyhow::Error;

use std::path::Path;
use utility::io::Parseable;

/// Majority Element
///
/// Given: A positive integer k≤20, a positive integer n≤10^4, and k arrays of size n containing positive integers not exceeding 10^5.
///
/// Return: For each array, output an element of this array occurring strictly more than n/2 times if such element exists, and "-1" otherwise.
pub fn rosalind_maj(filename: &Path) -> Result<Vec<isize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut parts = input.split('\n');
    let length_input = usize::parse_line(parts.next().unwrap())?;
    let (num_parts, length) = (length_input[0], length_input[1]);
    let mut elements = Vec::with_capacity(num_parts);
    for line in parts {
        let array = isize::parse_line(line)?;
        elements.push(find_majority_element(length, &array).unwrap_or(-1));
    }
    println!("{}", utility::io::format_array(&elements));
    Ok(elements)
}

/// Boyer–Moore majority vote algorithm
fn find_majority_element<T: PartialEq + PartialOrd + Copy>(
    length: usize,
    array: &[T],
) -> Option<T> {
    let mut count = 0;
    let mut majority_element = array[0];
    for element in array {
        if count == 0 {
            majority_element = *element;
        }
        if element == &majority_element {
            count += 1;
        } else {
            count -= 1;
        }
    }
    count = 0;
    for element in array {
        if element == &majority_element {
            count += 1;
        }
    }
    if count > length / 2 {
        Some(majority_element)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use utility::io::Parseable;

    use super::*;

    #[test]
    fn maj() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_maj")?;
        let output = isize::parse_line(&utility::io::input_from_file(&output_file)?)?;
        assert_eq!(rosalind_maj(&input_file)?, output);
        Ok(())
    }
}
