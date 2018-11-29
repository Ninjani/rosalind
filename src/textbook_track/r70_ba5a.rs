use crate::utils;
use std::usize;

/// Find the Minimum Number of Coins Needed to Make Change
///
/// Given: An integer money and an array Coins of positive integers.
///
/// Return: The minimum number of coins with denominations Coins that changes money.
pub fn rosalind_ba5a() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba5a.txt");
    let lines: Vec<_> = contents.split('\n').collect();
    let money = lines[0].parse::<usize>().unwrap();
    let coins: Vec<_> = lines[1]
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    println!("{}", get_change(money, &coins));
}

fn get_change(money: usize, coins: &[usize]) -> usize {
    let mut minimum_coins = vec![0];
    for m in 1..=money {
        minimum_coins.push(usize::MAX);
        for i in 0..coins.len() {
            if m >= coins[i] && minimum_coins[m - coins[i]] + 1 < minimum_coins[m] {
                minimum_coins[m] = minimum_coins[m - coins[i]] + 1;
            }
        }
    }
    minimum_coins[money]
}
