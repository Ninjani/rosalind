use std::collections::{btree_map::BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;
use std::num::{ParseFloatError, ParseIntError};

use failure::Error;
use itertools::Itertools;

use crate::utility::{
    errors,
    graph::{IntegerGraph, WeightedGraph},
};

const CODON_FILE: &str = "data/codons.txt";
const MASS_FILE: &str = "data/monoisotopic_mass.txt";
pub const STOP_CODON_AA: &str = "Stop";
pub const START_CODON: &str = "AUG";

/// Read problem input from file
pub fn input_from_file(filename: &str) -> Result<String, Error> {
    let mut f = File::open(filename)
        .map_err(|e| errors::RosalindParseError::FileReadError(e, filename.to_owned()))?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .map_err(|e| errors::RosalindParseError::FileReadError(e, filename.to_owned()))?;
    Ok(contents.trim().to_owned())
}

/// Read fasta-formatted file into a hashmap
pub fn read_fasta_file(filename: &str) -> Result<HashMap<String, String>, Error> {
    let contents = input_from_file(filename)?;
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
            Some(line) => sequences.insert(
                line[1..].to_owned(),
                fasta.next().ok_or_else(|| {
                    errors::RosalindParseError::BadFastaError(filename.to_owned())
                })?,
            ),
            None => break,
        };
    }
    Ok(sequences)
}

/// Read fasta-formatted file into a list of headers and a hashmap
pub fn read_fasta_file_and_headers(
    filename: &str,
) -> Result<(Vec<String>, HashMap<String, String>), Error> {
    let contents = input_from_file(filename)?;
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
                sequences.insert(
                    header_text,
                    fasta.next().ok_or_else(|| {
                        errors::RosalindParseError::BadFastaError(filename.to_owned())
                    })?,
                );
            }
            None => break,
        };
    }
    Ok((headers, sequences))
}

/// Codon to amino acid mapping
pub fn get_codon_to_aa() -> Result<HashMap<String, String>, Error> {
    let contents = input_from_file(CODON_FILE)?;
    let mut codons = HashMap::new();
    for line in contents.split('\n') {
        let mut codon_aas = line.split_whitespace();
        loop {
            let codon_aa = codon_aas.next();
            match codon_aa {
                Some(codon) => codons.insert(
                    codon.to_owned(),
                    codon_aas
                        .next()
                        .ok_or_else(|| errors::RosalindParseError::BadCodonFileError)?
                        .to_owned(),
                ),
                None => break,
            };
        }
    }
    Ok(codons)
}

/// Amino acid to codon mapping
pub fn get_aa_to_codon() -> Result<HashMap<String, Vec<String>>, Error> {
    let codon_to_aa = get_codon_to_aa()?;
    let mut aa_to_codon = HashMap::new();
    for (codon, aa) in codon_to_aa {
        aa_to_codon.entry(aa).or_insert_with(Vec::new).push(codon);
    }
    Ok(aa_to_codon)
}

/// Reads monoisotopic mass table
pub fn get_aa_to_mass() -> Result<HashMap<char, f64>, Error> {
    let mut mass_table = HashMap::new();
    let mass_contents = input_from_file(MASS_FILE)?;
    for line in mass_contents.split('\n') {
        let mut aa_mass = line.split_whitespace();
        if let (Some(aa), Some(mass)) = (aa_mass.next(), aa_mass.next()) {
            mass_table.insert(aa.chars().next().unwrap(), mass.parse::<f64>()?);
        }
    }
    Ok(mass_table)
}

pub fn get_mass_to_aa() -> Result<Vec<(f64, char)>, Error> {
    let mass_contents = input_from_file(MASS_FILE)?;
    let mut mass_aa = Vec::new();
    for line in mass_contents.split('\n') {
        let mut aa_mass = line.split_whitespace();
        if let (Some(aa), Some(mass)) = (aa_mass.next(), aa_mass.next()) {
            let mass = mass.parse::<f64>()?;
            let aa = aa
                .chars()
                .next()
                .ok_or_else(|| errors::RosalindOutputError::NoneError)?;
            mass_aa.push((mass, aa));
        }
    }
    mass_aa.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    Ok(mass_aa)
}

impl IntegerGraph {
    /// Reads in an adjacency_list of the form:
    /// ```
    /// node_1 -> node_2,node_3
    /// node_2 -> node_4
    /// ...
    /// ```
    pub fn from_adjacency_list(contents: &str, run_dfs: bool) -> Result<Self, Error> {
        let lines = contents.split('\n').filter(|line| !line.trim().is_empty());
        let mut adjacency_list = BTreeMap::new();
        let mut nodes = HashSet::new();
        for line in lines {
            let parts: Vec<_> = line.split(" -> ").collect();
            let node_1 = parts
                .get(0)
                .ok_or_else(|| {
                    errors::RosalindParseError::InputFormatError(String::from("Missing start_node"))
                })?
                .parse::<usize>()?;
            nodes.insert(node_1);
            let nodes_2 = parts
                .get(1)
                .ok_or_else(|| {
                    errors::RosalindParseError::InputFormatError(String::from("Missing end_nodes"))
                })?
                .split(',')
                .map(str::parse)
                .collect::<Result<Vec<usize>, _>>()?
                .into_iter()
                .collect::<Vec<_>>();
            for n in &nodes_2 {
                nodes.insert(*n);
            }
            adjacency_list.insert(node_1, nodes_2);
        }
        let mut nodes: Vec<_> = nodes.into_iter().collect();
        nodes.sort();
        Ok(Self::new(adjacency_list, nodes, run_dfs))
    }

    /// Reads a Rosalind edge list into an adjacency matrix
    /// ```
    /// num_nodes num_edges
    /// node_1 node_2
    /// node_1 node_2
    /// ...
    /// ```
    pub fn from_edge_list(
        lines: &mut dyn Iterator<Item=String>,
        directed: bool,
        run_dfs: bool,
    ) -> Result<Self, Error> {
        let length_input = lines
            .next()
            .ok_or_else(|| {
                errors::RosalindParseError::InputFormatError(String::from(
                    "Missing 'num_nodes num_edges' line",
                ))
            })?
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        let (num_nodes, num_edges) = (
            *length_input.get(0).ok_or_else(|| {
                errors::RosalindParseError::InputFormatError(String::from("Missing num_nodes"))
            })?,
            *length_input.get(1).ok_or_else(|| {
                errors::RosalindParseError::InputFormatError(String::from("Missing num_edges"))
            })?,
        );
        let mut adjacency_list = BTreeMap::new();
        let mut min_node = ::std::usize::MAX;
        let mut line;
        for _ in 0..num_edges {
            line = lines.next().ok_or_else(|| {
                errors::RosalindParseError::InputFormatError(String::from(
                    "Less lines than num_edges specifies",
                ))
            })?;
            let parts = line
                .split(' ')
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?;
            let (node_1, node_2) = (
                *parts.get(0).ok_or_else(|| {
                    errors::RosalindParseError::InputFormatError(String::from("Missing start_node"))
                })?,
                *parts.get(1).ok_or_else(|| {
                    errors::RosalindParseError::InputFormatError(String::from("Missing end_node"))
                })?,
            );
            if node_1 < min_node {
                min_node = node_1;
            }
            if node_2 < min_node {
                min_node = node_2;
            }
            {
                let edge_list_1 = adjacency_list.entry(node_1).or_insert_with(Vec::new);
                edge_list_1.push(node_2);
            }
            if !directed {
                let edge_list_2 = adjacency_list.entry(node_2).or_insert_with(Vec::new);
                edge_list_2.push(node_1);
            }
        }
        let nodes: Vec<_> = (min_node..min_node + num_nodes).collect();
        Ok(Self::new(adjacency_list, nodes, run_dfs))
    }
}

impl WeightedGraph {
    /// Read a Rosalind weighted edge list of the form:
    /// ```
    /// num_nodes num_edges
    /// node_1 node_2 weight
    /// node_1 node_2 weight
    /// ...
    /// ```
    pub fn from_weighted_edge_list(lines: &mut dyn Iterator<Item=String>) -> Result<Self, Error> {
        let length_input = lines
            .next()
            .ok_or_else(|| {
                errors::RosalindParseError::InputFormatError(String::from(
                    "Missing num_nodes num_edges line",
                ))
            })?
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        let (num_nodes, num_edges) = (
            *length_input.get(0).ok_or_else(|| {
                errors::RosalindParseError::InputFormatError(String::from("Missing num_nodes"))
            })?,
            *length_input.get(1).ok_or_else(|| {
                errors::RosalindParseError::InputFormatError(String::from("Missing num_edges"))
            })?,
        );
        let mut min_node = ::std::usize::MAX;
        let mut edges = Vec::with_capacity(num_edges);
        let mut adjacency_list = HashMap::with_capacity(num_nodes);
        for _ in 0..num_edges {
            let line = lines.next().ok_or_else(|| {
                errors::RosalindParseError::InputFormatError(String::from(
                    "Less lines than num_edges specifies",
                ))
            })?;
            let parts = line.split(' ').collect::<Vec<_>>();
            let node_1 = parts
                .get(0)
                .ok_or_else(|| {
                    errors::RosalindParseError::InputFormatError(String::from("Missing start_node"))
                })?
                .parse::<usize>()?;
            if node_1 < min_node {
                min_node = node_1;
            }
            let node_2 = parts
                .get(1)
                .ok_or_else(|| {
                    errors::RosalindParseError::InputFormatError(String::from("Missing end_node"))
                })?
                .parse::<usize>()?;
            if node_2 < min_node {
                min_node = node_2;
            }
            let weight = parts
                .get(2)
                .ok_or_else(|| {
                    errors::RosalindParseError::InputFormatError(String::from("Missing weight"))
                })?
                .parse::<i32>()?;
            edges.push((node_1, node_2, weight));
            let edge_list_1 = adjacency_list.entry(node_1).or_insert_with(Vec::new);
            edge_list_1.push((node_2, weight));
        }
        let nodes: Vec<_> = (min_node..min_node + num_nodes).collect();
        let node_to_index = nodes.iter().enumerate().map(|(i, n)| (*n, i)).collect();
        Ok(WeightedGraph {
            adjacency_list,
            edges,
            nodes,
            node_to_index,
            num_nodes,
            num_edges,
        })
    }
}

/// Trait to parse input
pub trait Parseable: Sized {
    /// Parse line as space-separated array
    fn parse_line(line: &str) -> Result<Vec<Self>, errors::RosalindParseError>;
}

impl Parseable for isize {
    fn parse_line(line: &str) -> Result<Vec<isize>, errors::RosalindParseError> {
        Ok(line
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, ParseIntError>>()?)
    }
}

impl Parseable for i32 {
    fn parse_line(line: &str) -> Result<Vec<i32>, errors::RosalindParseError> {
        Ok(line
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, ParseIntError>>()?)
    }
}

impl Parseable for i64 {
    fn parse_line(line: &str) -> Result<Vec<i64>, errors::RosalindParseError> {
        Ok(line
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, ParseIntError>>()?)
    }
}

impl Parseable for usize {
    fn parse_line(line: &str) -> Result<Vec<usize>, errors::RosalindParseError> {
        Ok(line
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, ParseIntError>>()?)
    }
}

impl Parseable for u64 {
    fn parse_line(line: &str) -> Result<Vec<u64>, errors::RosalindParseError> {
        Ok(line
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, ParseIntError>>()?)
    }
}

impl Parseable for u8 {
    fn parse_line(line: &str) -> Result<Vec<u8>, errors::RosalindParseError> {
        Ok(line
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, ParseIntError>>()?)
    }
}

impl Parseable for f64 {
    fn parse_line(line: &str) -> Result<Vec<f64>, errors::RosalindParseError> {
        Ok(line
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, ParseFloatError>>()?)
    }
}

impl Parseable for f32 {
    fn parse_line(line: &str) -> Result<Vec<f32>, errors::RosalindParseError> {
        Ok(line
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, ParseFloatError>>()?)
    }
}

impl Parseable for char {
    fn parse_line(line: &str) -> Result<Vec<char>, errors::RosalindParseError> {
        Ok(line
            .trim()
            .split_whitespace()
            .map(|s| {
                s.chars().next().ok_or_else(|| {
                    errors::RosalindParseError::InputFormatError(String::from("Empty string"))
                })
            })
            .collect::<Result<Vec<_>, _>>()?)
    }
}

/// Read file of form:
/// ```
/// length
/// a1 a2 a3 ...
/// ```
pub fn read_isize_array(filename: &str) -> Result<(usize, Vec<isize>), Error> {
    let contents = input_from_file(filename)?;
    let lines = contents.split('\n').collect::<Vec<_>>();
    let length = lines
        .get(0)
        .ok_or_else(|| {
            errors::RosalindParseError::InputFormatError(String::from("Missing length line"))
        })?
        .parse::<usize>()?;
    let array = isize::parse_line(lines.get(1).ok_or_else(|| {
        errors::RosalindParseError::InputFormatError(String::from("Missing array line"))
    })?)?;
    Ok((length, array))
}

/// Read set of form:
/// {n1, n2, n3, ... }
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

/// Input iterator to "separator"-separated string of items
pub fn format_line<T: ToString>(items: impl Iterator<Item=T>, separator: &str) -> String {
    items
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(separator)
}

/// Format space-separated array
pub fn format_array<T: ToString>(input: &[T]) -> String {
    input
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Format comma-separated set
pub fn format_set<T: ToString + ::std::cmp::Eq + ::std::hash::Hash>(input: &HashSet<T>) -> String {
    format!(
        "{{{}}}",
        input
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    )
}
