use crate::textbook_track::r74_ba5e::{read_scoring_matrix, AlignmentParameters};
use crate::textbook_track::r80_ba5k::LinearSpaceAlignment;
use crate::utils;



/// W.I.P

pub fn rosalind_ba5l() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba5l.txt");
    let lines: Vec<_> = contents.split('\n').collect();
    let (scoring_matrix, amino_acids) = read_scoring_matrix("data/blosum62.txt");
//    scoring_matrix.fill(-1);
//    scoring_matrix.diag_mut().assign(&Array::from_vec(
//        (0..amino_acids.len()).map(|_| 1).collect(),
//    ));
    let parameters = AlignmentParameters::new(scoring_matrix, amino_acids, 5);
    let lsa = LinearSpaceAlignment {
        string_1: lines[0].chars().collect(),
        string_2: lines[1].chars().collect(),
        parameters,
    };
    let directions = lsa.align(0, lsa.string_1.len(), 0, lsa.string_2.len());
    let (mut chars_1, mut chars_2) = (lines[0].chars(), lines[1].chars());
    let aln_string_1 = directions.iter().map(|d| match d {
        1 | 3 => chars_1.next().unwrap(),
        2 => '-',
        _ => panic!("Unknown direction"),
    }).collect::<String>();
    let aln_string_2 = directions.iter().map(|d| match d {
        2 | 3 => chars_2.next().unwrap(),
        1 => '-',
        _ => panic!("Unknown direction"),
    }).collect::<String>();
    println!("{}\n{}", aln_string_1, aln_string_2);
}

//LinearSpaceAlignment(top, bottom, left, right)
//        if left = right
//            return alignment formed by bottom − top vertical edges
//        if top = bottom
//            return alignment formed by right − left horizontal edges
//        middle ← ⌊ (left + right)/2⌋
//        midNode ← MiddleNode(top, bottom, left, right)
//        midEdge ← MiddleEdge(top, bottom, left, right)
//        LinearSpaceAlignment(top, midNode, left, middle)
//        output midEdge
//        if midEdge = "→" or midEdge = "↘"
//            middle ← middle + 1
//        if midEdge = "↓" or midEdge ="↘"
//            midNode ← midNode + 1
//        LinearSpaceAlignment(midNode, bottom, middle, right)

impl LinearSpaceAlignment {
    pub fn align(&self, top: usize, bottom: usize, left: usize, right: usize) -> Vec<usize>{
        if left == right {
            return (0..bottom - top).map(|_| 1).collect()
        }
        if top == bottom {
            return (0..right - left).map(|_| 2).collect();
        }
        let mut middle = ((left + right) as f32 / 2.) as usize;
        let (mut middle_node, direction) = self.get_middle_node_and_edge(top, bottom, left, right);
        let mut alignment = self.align(top, middle_node, left, middle);
        alignment.push(direction);
        if (direction == 2) | (direction == 3) {
            middle += 1;
        }
        if (direction == 1) | (direction == 3) {
            middle_node += 1;
        }
        alignment.extend_from_slice(&self.align(middle_node, bottom, middle, right));
        alignment
    }
}
