use crate::utils;
use crate::utils::Parseable;
use failure::Error;

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

/// Majority Element
///
/// Given: A positive integer k≤20, a positive integer n≤10^4, and k arrays of size n containing positive integers not exceeding 10^5.
///
/// Return: For each array, output an element of this array occurring strictly more than n/2 times if such element exists, and "-1" otherwise.
pub fn rosalind_maj() -> Result<(), Error> {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_maj.txt");
    let mut parts = contents.split('\n');
    let length_input = usize::parse_line(parts.next().unwrap())?;
    let (_, length) = (length_input[0], length_input[1]);
    for line in parts {
        let array = isize::parse_line(line)?;
        print!("{} ", find_majority_element(length, &array).unwrap_or(-1))
    }
    Ok(())
}
