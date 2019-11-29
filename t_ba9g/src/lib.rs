use failure::Error;

use utility;

/// Construct the suffix array of a string.
///
/// Given: A string Text.
///
/// Return: SuffixArray(Text).
pub fn rosalind_ba9g(filename: &str) -> Result<Vec<usize>, Error> {
    let text = utility::io::input_from_file(filename)?;
    let suffix_array = SuffixArray::construct(&text);
    println!(
        "{}",
        suffix_array
            .suffix_array
            .iter()
            .map(|s| format!("{}", s))
            .collect::<Vec<_>>()
            .join(", ")
    );
    Ok(suffix_array.suffix_array)
}

pub struct SuffixArray {
    pub suffix_array: Vec<usize>,
    pub text: String,
}

impl SuffixArray {
    pub fn construct(text: &str) -> Self {
        let mut suffixes = Vec::new();
        for i in 0..text.len() {
            suffixes.push((text[i..text.len()].to_owned(), i));
        }
        suffixes.sort();
        SuffixArray {
            suffix_array: suffixes.into_iter().map(|(_, i)| i).collect(),
            text: text.to_owned(),
        }
    }
}

//impl From<SuffixTree> for SuffixArray {
//    fn from(suffix_tree: SuffixTree) -> Self {
//
//    }
//}
//
//impl SuffixTree {
//    fn _preorder(&self, node: NodeIndex<u32>) {
//        *visited.entry(self.root).or_insert(false) = true;
//        for node_2 in self.tree.edges(node).map(|e| e.source())
//    }
//
//    // PREORDER(Tree, Node)
//    //visit Node
//    //for each child Node’ of Node from left to right
//    //PREORDER(Tree,Node’)
//    fn preorder(&self) {
//        let mut visited: HashMap<_, _> = self.tree.node_indices().map(|n| (n, false)).collect();
//
//    }
//
//}
