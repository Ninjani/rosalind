use std::collections::HashMap;

use failure::Error;

use utility;

pub fn rosalind_ba9j(filename: &str) -> Result<String, Error> {
    let bwt_string = utility::io::input_from_file(filename)?;
    let input_string = inverse_bwt(&bwt_string)?;
    println!("{}", input_string);
    Ok(input_string)
}

fn inverse_bwt(text: &str) -> Result<String, Error> {
    fn get_char_index_from_index(string: &[char], character: char, index: usize) -> Option<usize> {
        let mut char_index = 0;
        for (i, c) in string.iter().enumerate() {
            if i == index && *c == character {
                return Some(char_index);
            }
            if *c == character {
                char_index += 1;
            }
        }
        None
    }

    let mut first_column: Vec<_> = text.chars().collect();
    first_column.sort();
    let last_column: Vec<_> = text.chars().collect();
    let mut mapping_last = HashMap::new();
    for (i, c) in last_column.iter().enumerate() {
        mapping_last.entry(*c).or_insert_with(Vec::new).push(i);
    }
    let mut input_string = String::with_capacity(text.len());
    let mut last_index = last_column.iter().position(|a| *a == '$').unwrap();
    let mut first_char = first_column[last_index];
    input_string.push(first_char);
    while input_string.len() < text.len() {
        let char_index = get_char_index_from_index(&first_column, first_char, last_index).unwrap();
        last_index = mapping_last[&first_char][char_index];
        first_char = first_column[last_index];
        input_string.push(first_char);
    }
    Ok(input_string)
}
