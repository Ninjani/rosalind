use std::usize;

use failure::Error;

use utility;

/// Find the Minimum Number of Coins Needed to Make Change
///
/// Given: An integer money and an array Coins of positive integers.
///
/// Return: The minimum number of coins with denominations Coins that changes money.
pub fn rosalind_ba5a(filename: &str) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let lines: Vec<_> = contents.split('\n').collect();
    let money = lines[0].parse::<usize>()?;
    let coins: Vec<_> = lines[1]
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;
    println!("{}", get_change(money, &coins));
    Ok(())
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
