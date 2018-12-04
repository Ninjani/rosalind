use crate::utils;
use crate::utils::Parseable;
use failure::Error;

pub fn rosalind_pdpl() -> Result<(), Error> {
    let mut difference_multiset =
        usize::parse_line(&utils::input_from_file("data/stronghold/rosalind_pdpl.txt"))?;
    difference_multiset.sort();
    let mut set = vec![0];
    let n = (1 + (1 + (8. * difference_multiset.len() as f64).sqrt() as usize)) / 2;
    for i in 0..(n - 1) {
        let c = set[i];
        set.push(c + difference_multiset[i]);
    }
    utils::print_array(&set);
    Ok(())
}
