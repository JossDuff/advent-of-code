use std::fs::read_to_string;
use std::str::FromStr;

use hand_wager::HandWager;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

mod hand;
mod hand_wager;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut hand_wagers: Vec<HandWager> = input
        .lines()
        .map(|s| HandWager::from_str(s).unwrap())
        .collect();

    // sorts lowest to highest
    hand_wagers.sort();
    let total_winnings: u64 = hand_wagers
        .into_par_iter()
        .enumerate()
        .map(|(i, hand_wager)| (i + 1) as u64 * hand_wager.wager)
        .sum();

    println!("total winnings: {total_winnings}");
}
