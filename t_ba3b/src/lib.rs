use failure::Error;

use utility;

pub fn rosalind_ba3b() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba3b.txt")?;
    let kmers: Vec<_> = contents.split('\n').collect();
    println!("{}", reverse_kmerize(&kmers));
    Ok(())
}

pub fn reverse_kmerize(kmers: &[&str]) -> String {
    let k = &kmers[0].len();
    let first: String = kmers[0].chars().collect();
    let end = kmers[1..]
        .iter()
        .flat_map(|kmer| kmer.chars().skip(k - 1))
        .collect::<String>();
    format!("{}{}", first, end)
}
