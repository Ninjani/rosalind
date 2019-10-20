use failure::Error;
use ndarray::{Array1, Array2};

use crate::textbook_track::r74_ba5e::{AlignmentParameters, read_scoring_matrix};
use crate::utility;

/// Find a Middle Edge in an Alignment Graph in Linear Space
///
/// Given: Two amino acid strings.
///
/// Return: A middle edge in the alignment graph of these strings,
/// where the optimal path is defined by the BLOSUM62 scoring matrix
/// and a linear indel penalty equal to 5.
/// Return the middle edge in the form “(i, j) (k, l)”,
/// where (i, j) connects to (k, l).
pub fn rosalind_ba5k() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba5k.txt")?;
    let lines: Vec<_> = contents.split('\n').collect();
    let (scoring_matrix, amino_acids) = read_scoring_matrix("data/blosum62.txt")?;
    let parameters = AlignmentParameters::new(scoring_matrix, amino_acids, 5);
    let lsa = LinearSpaceAlignment {
        string_1: lines[0].chars().collect(),
        string_2: lines[1].chars().collect(),
        parameters,
    };
    let (middle_node, direction) =
        lsa.get_middle_node_and_edge(0, lsa.string_1.len(), 0, lsa.string_2.len());
    let middle = ((lsa.string_2.len() as f32) / 2.).floor() as usize;
    let (start, end) = match direction {
        1 => ((middle_node, middle), (middle_node + 1, middle)),
        2 => ((middle_node, middle), (middle_node, middle + 1)),
        3 => ((middle_node, middle), (middle_node + 1, middle + 1)),
        _ => panic!("no/invalid backtracking direction set"),
    };
    println!("{:?} {:?}", start, end);
    Ok(())
}

pub struct LinearSpaceAlignment {
    pub string_1: Vec<char>,
    pub string_2: Vec<char>,
    pub parameters: AlignmentParameters,
}

impl LinearSpaceAlignment {
    pub fn get_scores(
        &self,
        top: usize,
        bottom: usize,
        left: usize,
        right: usize,
        reverse: bool,
    ) -> (Array1<isize>, Array1<usize>) {
        let (n, m) = (bottom - top + 1, right - left + 1);
        let mut scores = Array2::zeros((2, n));
        let mut backtrack = Array1::zeros(n);
        scores[(1, 0)] = scores[(0, 0)] - self.parameters.gap_penalty;
        backtrack[0] = 1;
        for j in 1..n {
            scores[(0, j)] = scores[(0, j - 1)] - self.parameters.gap_penalty;
        }
        let mut i_index = 1;
        let mut i_1_index;
        for i in 1..m {
            i_index = i % 2;
            i_1_index = (i + 1) % 2;
            for j in 1..n {
                let values: Vec<isize> = vec![
                    (scores[(i_1_index, j)] - self.parameters.gap_penalty),
                    (scores[(i_index, j - 1)] - self.parameters.gap_penalty),
                    (scores[(i_1_index, j - 1)]
                        + if reverse {
                        self.parameters.scoring_matrix[(
                            self.parameters.amino_acid_order[&self.string_1[bottom - j]],
                            self.parameters.amino_acid_order[&self.string_2[right - i]],
                        )]
                    } else {
                        self.parameters.scoring_matrix[(
                            self.parameters.amino_acid_order[&self.string_1[top + j - 1]],
                            self.parameters.amino_acid_order[&self.string_2[left + i - 1]],
                        )]
                    }),
                ];
                let (max_index, max_value) = values
                    .into_iter()
                    .enumerate()
                    .max_by(|a, b| a.1.cmp(&b.1))
                    .unwrap();
                scores[(i_index, j)] = max_value;
                backtrack[j] = max_index + 1;
            }
        }
        (scores.row(i_index).to_owned(), backtrack)
    }

    pub fn get_middle_node_and_edge(
        &self,
        top: usize,
        bottom: usize,
        left: usize,
        right: usize,
    ) -> (usize, usize) {
        let middle = ((left + right) as f32 / 2.) as usize;
        let (scores_from_source, _) = self.get_scores(top, bottom, left, middle, false);
        let (scores_to_sink, backtrack_to_sink) = self.get_scores(top, bottom, middle, right, true);
        let max_node = (top..bottom)
            .map(|i| (scores_from_source[i - top] + scores_to_sink[bottom - i], i))
            .max()
            .unwrap()
            .1;
        (max_node, backtrack_to_sink[bottom - max_node])
    }
}
