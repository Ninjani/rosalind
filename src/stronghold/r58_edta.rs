use utils;
use stronghold::r46_edit::get_edit_distance;
use ndarray::Array2;

/// WIP
/// Edit Distance Alignment
///
/// Given: Two protein strings s and t in FASTA format (with each string having length at most 1000 aa).
///
/// Return: The edit distance dE(s,t) followed by two augmented strings s′ and t′ representing an optimal alignment of s and t.
pub fn rosalind_edta() {
    let fasta = utils::read_fasta_file("data/stronghold/rosalind_edta.txt");
    let sequences: Vec<String> = fasta.values().map(|x| x.to_owned()).collect();
    let (string_1, string_2) = (&sequences[0], &sequences[1]);
    let distances = get_edit_distance(string_1, string_2);
    let (aln_1, aln_2) = backtrack(string_1, string_2, &distances);
    println!("{}\n{}\n{}", distances[(string_1.len(), string_2.len())], aln_1, aln_2);
}

fn backtrack(string_1: &str, string_2: &str, distances: &Array2<usize>) -> (String, String) {
    let (string_1, string_2): (Vec<_>, Vec<_>) = (string_1.chars().collect(), string_2.chars().collect());
    let (mut m, mut n) = (string_1.len(), string_2.len());
    let (mut aln_1, mut aln_2) = (Vec::new(), Vec::new());
    loop {
        if m == 0 && n == 0 {
            break
        } else if n == 0 {
            m -= 1;
            aln_1.push(string_1[m]);
            aln_2.push('-')
        } else if n == 0 {
            n -= 1;
            aln_1.push('-');
            aln_2.push(string_2[n]);
        } else {
            let indices = [(m-1, n-1), (m-1, n), (m, n-1)];
            let (min_index, min_distance) = indices
                .into_iter()
                .enumerate()
                .map(|(i, x)| (i, distances[*x]))
                .min_by(|a, b| a.1.cmp(&b.1)).unwrap();
            println!("{} {} {:?} {} {} {}", m, n, indices[min_index], min_distance, string_1[m-1], string_2[n-1]);
            if indices[min_index].0 == m - 1 {
                aln_1.push(string_1[m - 1]);
            } else {
                aln_1.push('-');
            }
            if indices[min_index].1 == n - 1 {
                aln_2.push(string_2[n - 1]);
            } else {
                aln_2.push('-');
            }
            m = indices[min_index].0;
            n = indices[min_index].1;
        }
    }
    (aln_1.into_iter().rev().collect(), aln_2.into_iter().rev().collect())
}
