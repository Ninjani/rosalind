use failure::Error;

use crate::textbook_track::r114_ba9g::SuffixArray;
use crate::utility;

/// Pattern Matching with the Suffix Array
///
/// Given: A string Text and a collection of strings Patterns.
///
/// Return: All starting positions in Text where a string from Patterns appears as a substring.
pub fn rosalind_ba9h(filename: &str) -> Result<Vec<usize>, Error> {
    let content = utility::io::input_from_file(filename)?;
    let mut lines = content.split('\n');
    let text = lines.next().unwrap().to_owned();
    let suffix_array = SuffixArray::construct(&text);
    let mut positions = Vec::new();
    for pattern in lines {
        if let Some((first, last)) = suffix_array.pattern_match(pattern.trim()) {
            positions.extend((first..=last).map(|i| suffix_array.suffix_array[i]));
        }
    }
    println!("{}", utility::io::format_array(&positions));
    Ok(positions)
}

impl SuffixArray {
    fn pattern_match(&self, pattern: &str) -> Option<(usize, usize)> {
        let mut min_index = 0;
        let mut max_index = self.text.len();
        let mut mid_index;
        while min_index < max_index {
            mid_index = (min_index + max_index) / 2;
            if pattern > &self.text[self.suffix_array[mid_index]..] {
                min_index = mid_index + 1;
            } else {
                max_index = mid_index;
            }
        }
        let first = min_index;
        max_index = self.text.len();
        while min_index < max_index {
            mid_index = (min_index + max_index) / 2;
            if pattern < &self.text[self.suffix_array[mid_index]..] {
                max_index = mid_index;
            } else {
                min_index = mid_index + 1;
            }
        }
        let last = max_index;
        if first > last {
            None
        } else {
            Some((first, last))
        }
    }
}
