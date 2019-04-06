use crate::algorithmic_heights::r24_med::partition;
use crate::utils;
use crate::utils::Parseable;
use failure::Error;
use rand::{thread_rng, Rng};

pub fn rosalind_qs() -> Result<(), Error> {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_qs.txt");
    let lines: Vec<_> = contents.split('\n').collect();
    let mut array = isize::parse_line(lines[1])?;
    quicksort(&mut array);
    utils::print_array(&array);
    Ok(())
}

fn quicksort(array: &mut [isize]) {
    let mut pivot_index = thread_rng().gen_range(0, array.len());
    pivot_index = partition(array, 0, array.len() - 1, pivot_index);
    if pivot_index == 0 {
        pivot_index += 1;
    }
    let (mut left_array, mut right_array) = array.split_at_mut(pivot_index);
    if left_array.len() > 1 {
        quicksort(&mut left_array);
    }
    if right_array.len() > 1 {
        quicksort(&mut right_array);
    }
}
