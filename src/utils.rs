use bio::pattern_matching::{bom, shift_and};
use failure::Error;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use num::BigUint;
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;
use std::num::{ParseFloatError, ParseIntError};
use std::collections::btree_map::BTreeMap;
use std::string::ToString;
const CODON_FILE: &str = "data/codons.txt";
pub const STOP_CODON_AA: &str = "Stop";
pub const START_CODON: &str = "AUG";

/// Read problem input from file
pub fn input_from_file(filename: &str) -> String {
    let mut f = File::open(filename).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    contents.trim().to_owned()
}

/// Input iterator to "separator"-separated string of items
pub fn format_line<T: ToString>(items: impl Iterator<Item = T>, separator: &str) -> String {
    items.map(|c| c.to_string()).collect::<Vec<_>>().join(separator)
}

/// Print a space-separated array
pub fn print_array<T: ToString>(input: &[T]) {
    println!(
        "{}",
        input.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" ")
    );
}

/// Print a comma-separated set
pub fn print_set<T: ToString + ::std::cmp::Eq + ::std::hash::Hash>(
    input: &HashSet<T>,
) {
    println!(
        "{{{}}}",
        input.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", ")
    );
}

/// Count occurrences of each character in a string
pub fn char_counter(input: &str) -> HashMap<char, usize> {
    let mut counter = HashMap::new();
    for character in input.chars() {
        if let Some(value) = counter.get_mut(&character) {
            *value += 1;
            continue;
        }
        counter.insert(character, 1usize);
    }
    counter
}

/// Read fasta-formatted content into a hashmap
pub fn read_fasta(contents: &str) -> HashMap<String, String> {
    let groups = contents
        .split('\n')
        .collect::<Vec<&str>>()
        .into_iter()
        .group_by(|line| line.starts_with('>'));
    let mut fasta = groups
        .into_iter()
        .map(|(_, group)| group.collect::<Vec<&str>>().join(""));
    let mut sequences = HashMap::new();
    loop {
        let header = fasta.next();
        match header {
            Some(line) => sequences.insert(line[1..].to_owned(), fasta.next().unwrap()),
            None => break,
        };
    }
    sequences
}

/// Read fasta-formatted file into a list of headers and a hashmap
pub fn read_fasta_file_and_headers(filename: &str) -> (Vec<String>, HashMap<String, String>) {
    let contents = input_from_file(filename);
    let groups = contents
        .split('\n')
        .collect::<Vec<&str>>()
        .into_iter()
        .group_by(|line| line.starts_with('>'));
    let mut fasta = groups
        .into_iter()
        .map(|(_, group)| group.collect::<Vec<&str>>().join(""));
    let mut headers = Vec::new();
    let mut sequences = HashMap::new();
    loop {
        let header = fasta.next();
        match header {
            Some(line) => {
                let header_text = line[1..].to_owned();
                headers.push(header_text.clone());
                sequences.insert(header_text, fasta.next().unwrap());
            }
            None => break,
        };
    }
    (headers, sequences)
}

/// Read fasta-formatted file into a hashmap
pub fn read_fasta_file(filename: &str) -> HashMap<String, String> {
    let contents = input_from_file(filename);
    read_fasta(&contents)
}

/// Exact string search for overlapping motifs in a string (No regex).
pub fn find_motifs(motif: &str, string: &str) -> Vec<usize> {
    if motif.len() < 64 {
        let matcher = shift_and::ShiftAnd::new(motif.as_bytes());
        matcher.find_all(string.as_bytes()).collect()
    } else {
        let matcher = bom::BOM::new(motif.as_bytes());
        matcher.find_all(string.as_bytes()).collect()
    }
}

/// Codon to amino acid mapping
pub fn get_codon_to_aa() -> HashMap<String, String> {
    let contents = input_from_file(CODON_FILE);
    let mut codons = HashMap::new();
    for line in contents.split('\n') {
        let mut codon_aas = line.split_whitespace();
        loop {
            let codon_aa = codon_aas.next();
            match codon_aa {
                Some(codon) => {
                    codons.insert(codon.to_owned(), codon_aas.next().unwrap().to_owned())
                }
                None => break,
            };
        }
    }
    codons
}

/// Amino acid to codon mapping
pub fn get_aa_to_codon() -> HashMap<String, Vec<String>> {
    let codon_to_aa = get_codon_to_aa();
    let mut aa_to_codon = HashMap::new();
    for (codon, aa) in codon_to_aa {
        aa_to_codon.entry(aa).or_insert_with(Vec::new).push(codon);
    }
    aa_to_codon
}

/// Chunk a string into sub-strings
pub fn sub_strings(source: &str, sub_size: usize) -> Vec<String> {
    source
        .chars()
        .chunks(sub_size)
        .into_iter()
        .map(|c| c.collect())
        .collect()
}

/// Permutation $^nC_r$
pub fn ncr(n: u64, r: u64) -> BigUint {
    let r = r.min(n - r);
    if r == 0 {
        return BigUint::from(1u64);
    }
    let numerator: BigUint = ((n - r + 1)..=n).product();
    let denominator: BigUint = (1..=r).product();
    numerator / denominator
}

pub fn factorial(n: usize) -> BigUint {
    (1..=n).product()
}

/// Reads in a Rosalind edge list of the form:
/// ```
/// num_nodes num_edges
/// node_1 node_2
/// node_1 node_2
/// ...
/// ```
pub fn read_edge_list(lines: &mut Iterator<Item = String>, substract: bool) -> (usize, usize, Vec<(usize, usize)>) {
    let length_input = lines
        .next()
        .unwrap()
        .split(' ')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let (num_nodes, num_edges) = (length_input[0], length_input[1]);
    let mut edges = Vec::with_capacity(num_edges);
    let mut line;
    for _ in 0..num_edges {
        line = lines.next().unwrap();
        let parts = line
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        if substract {
            edges.push((parts[0] - 1, parts[1] - 1));
        } else {
            edges.push((parts[0], parts[1]));
        }

    }
    (num_nodes, num_edges, edges)
}

/// Reads in an adjacency_list of the form:
/// ```
/// node_1 -> node_2,node_3
/// node_2 -> node_4
/// ...
/// ```
pub fn read_adjacency_list(
    contents: &str,
    zero_start: bool,
) -> Result<(usize, BTreeMap<usize, Vec<usize>>), Error> {
    let lines = contents.split('\n');
    let mut adjacency_list = BTreeMap::new();
    let mut num_nodes = 0;
    for line in lines {
        let parts: Vec<_> = line.split(" -> ").collect();
        let mut node_1 = parts[0].parse::<usize>()?;
        if zero_start {
            node_1 += 1;
        }
        if node_1 > num_nodes {
            num_nodes = node_1;
        }
        let nodes_2 = parts[1]
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<usize>, _>>()?
            .into_iter()
            .map(|x| if zero_start { x + 1 } else { x })
            .collect::<Vec<_>>();
        for n in &nodes_2 {
            if *n > num_nodes {
                num_nodes = *n;
            }
        }
        adjacency_list.insert(node_1, nodes_2);
    }
    Ok((num_nodes, adjacency_list))
}

/// Reads a Rosalind edge list into an adjacency matrix
/// ```
/// num_nodes
/// node_1 node_2
/// node_1 node_2
/// ...
/// ```
pub fn read_edge_list_to_adjacency_list(
    contents: &str,
    directed: bool,
    subtract: bool
) -> Result<(usize, BTreeMap<usize, Vec<usize>>), Error> {
    let mut lines = contents.split('\n');
    let num_nodes = lines.next().unwrap().parse::<usize>()?;
    let mut adjacency_matrix = BTreeMap::new();
    for line in lines {
        let parts = line
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        let mut node_1 = parts[0];
        let mut node_2 = parts[1];
        if subtract {
            node_1 -= 1;
            node_2 -= 1;
        }
        {
            let edge_list_1 = adjacency_matrix.entry(node_1).or_insert_with(Vec::new);
            edge_list_1.push(node_2);
        }
        if !directed {
            let edge_list_2 = adjacency_matrix.entry(node_2).or_insert_with(Vec::new);
            edge_list_2.push(node_1);
        }
    }
    Ok((num_nodes, adjacency_matrix))
}

/// Read a Rosalind weighted edge list of the form:
/// ```
/// num_nodes num_edges
/// node_1 node_2 weight
/// node_1 node_2 weight
/// ...
/// ```
pub fn read_weighted_edge_list(
    lines: &mut Iterator<Item = String>,
    subtract: bool
) -> Result<(usize, usize, Vec<(usize, usize, isize)>), Error> {
    let length_input = lines
        .next()
        .unwrap()
        .split(' ')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let (num_nodes, num_edges) = (length_input[0], length_input[1]);
    let mut edges = Vec::with_capacity(num_edges);
    for _ in 0..num_edges {
        let line = lines.next().unwrap();
        let parts = line.split(' ').collect::<Vec<_>>();
        let mut node_1 = parts[0].parse::<usize>()?;
        let mut node_2 = parts[1].parse::<usize>()?;
        if subtract {
            node_1 -= 1;
            node_2 -= 1;
        }
        let weight = parts[2].parse::<isize>()?;
        edges.push((node_1, node_2, weight));
    }
    Ok((num_nodes, num_edges, edges))
}

#[derive(Debug, Fail)]
#[fail(display = "ParseError")]
/// ParseIntError or ParseFloatError
pub struct ParseError;

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        ParseError
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(_: ParseFloatError) -> Self {
        ParseError
    }
}

/// Trait to parse input
pub trait Parseable: Sized {
    fn parse_line(line: &str) -> Result<Vec<Self>, ParseError>;
}

impl Parseable for isize {
    /// Parse line as space-separated array
    fn parse_line(line: &str) -> Result<Vec<isize>, ParseError> {
        Ok(line
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, ParseIntError>>()?)
    }
}

impl Parseable for usize {
    fn parse_line(line: &str) -> Result<Vec<usize>, ParseError> {
        Ok(line
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, ParseIntError>>()?)
    }
}

impl Parseable for u64 {
    fn parse_line(line: &str) -> Result<Vec<u64>, ParseError> {
        Ok(line
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, ParseIntError>>()?)
    }
}

impl Parseable for u8 {
    fn parse_line(line: &str) -> Result<Vec<u8>, ParseError> {
        Ok(line
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, ParseIntError>>()?)
    }
}

impl Parseable for f64 {
    fn parse_line(line: &str) -> Result<Vec<f64>, ParseError> {
        Ok(line
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, ParseFloatError>>()?)
    }
}

/// Read file of form:
/// ```
/// length
/// a1 a2 a3 ...
/// ```
pub fn read_isize_array(filename: &str) -> Result<(usize, Vec<isize>), Error> {
    let contents = input_from_file(filename);
    let lines = contents.split('\n').collect::<Vec<_>>();
    let length = lines[0].parse::<usize>()?;
    let array = isize::parse_line(lines[1])?;
    Ok((length, array))
}

/// Parse a line of whitespace separated characters
pub fn parse_chars(line: &str) -> Vec<char> {
    line.trim()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<_>>()
}

/// max/min argmax/argmin for floats
pub trait Comparable: Sized {
    fn array_max(array: &[Self]) -> Self;
    fn array_min(array: &[Self]) -> Self;
    fn array_index_max(array: &[Self]) -> (usize, Self);
    fn array_index_min(array: &[Self]) -> (usize, Self);
}

impl Comparable for f64 {
    fn array_max(array: &[Self]) -> Self {
        array.to_vec().into_iter().fold(::std::f64::NAN, f64::max)
    }
    fn array_min(array: &[Self]) -> Self {
        array.to_vec().into_iter().fold(0., f64::min)
    }
    fn array_index_max(array: &[Self]) -> (usize, Self) {
        let mut max_index = 0;
        let mut max_value = ::std::f64::MIN;
        for (i, a) in array.to_vec().into_iter().enumerate() {
            if a > max_value {
                max_index = i;
                max_value = a;
            }
        }
        (max_index, max_value)
    }
    fn array_index_min(array: &[Self]) -> (usize, Self) {
        let mut min_index = 0;
        let mut min_value = ::std::f64::MAX;
        for (i, a) in array.to_vec().into_iter().enumerate() {
            if a < min_value {
                min_index = i;
                min_value = a;
            }
        }
        (min_index, min_value)
    }
}

pub fn read_set(line: &str) -> Result<HashSet<usize>, Error> {
    let chars: Vec<_> = line.chars().collect();
    let line: String = chars[1..(line.len() - 1)].iter().collect();
    Ok(HashSet::from_iter(
        line.split(", ")
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter(),
    ))
}

pub fn set_pop<T: ::std::hash::Hash + Eq + Clone>(set: &mut HashSet<T>) -> Option<T> {
    match set.iter().next() {
        Some(x) => set.take(&x.clone()),
        None => None
    }
}

/// Return overlapping kmers of a given length from a string
pub fn kmerize(string: &str, length: usize) -> Vec<String> {
    string
        .chars()
        .collect::<Vec<_>>()
        .windows(length)
        .map(|chunk| chunk.iter().cloned().collect::<String>())
        .collect()
}
