use failure::Error;

use crate::utility;

/// Speeding Up Motif Finding
///
/// Given: A DNA string s (of length at most 100 kbp) in FASTA format.
///
/// Return: The failure array of s.
pub fn rosalind_kmp(filename: &str) -> Result<Vec<Vec<isize>>, Error> {
    let dna = utility::io::read_fasta_file(filename)?;
    let mut counts: Vec<Vec<_>> = Vec::with_capacity(dna.len());
    for (_, sequence) in dna {
        let array = compute_failure_array(&sequence);
        println!("{}", utility::io::format_array(&array));
        counts.push(array);
    }
    Ok(counts)
}

fn compute_failure_array(string: &str) -> Vec<isize> {
    let n = string.len();
    let characters: Vec<_> = string.chars().collect();
    let mut failure_array: Vec<isize> = (0..=n).map(|_| -1).collect();
    let mut j = -1isize;
    for i in 1..=n {
        while (j >= 0) && (characters[i - 1] != characters[(j as usize)]) {
            j = failure_array[j as usize];
        }
        j += 1;
        failure_array[i] = j;
    }
    failure_array[1..].to_vec()
}

#[cfg(test)]
mod tests {
    use crate::utility::io::Parseable;

    use super::*;

    #[test]
    fn kmp() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_kmp")?;
        let output = utility::io::input_from_file(&output_file)?
            .split('\n')
            .map(|line| isize::parse_line(line))
            .collect::<Result<Vec<Vec<_>>, _>>()?;
        assert_eq!(rosalind_kmp(&input_file)?, output);
        Ok(())
    }
}
