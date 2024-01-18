use std::{fs::read_to_string, u32::MAX};

fn main() {
    let input = read_to_string("input.txt").unwrap();

    // start with one copy of card 1 (0, 1)
    let mut card_count: Vec<u32> = vec![1; 209];

    // let _ = input
    //     .lines()
    //     .map(|line| obey_the_elf(line, &mut card_count));

    for line in input.lines() {
        obey_the_elf(line, &mut card_count);
    }

    let sum: u32 = card_count.iter().sum();
    println!("{sum}");
}

fn obey_the_elf(line: &str, card_count: &mut Vec<u32>) {
    let initial_split = line.split(":").collect::<Vec<&str>>();

    let card_number: u32 = if let Some(x) = initial_split.first().unwrap().split_whitespace().last()
    {
        x.parse().unwrap()
    } else {
        return;
    };

    let card_copies: u32 = *card_count.get((card_number - 1) as usize).unwrap_or(&0);

    let only_numbers = initial_split
        .last()
        .unwrap()
        .split("|")
        .collect::<Vec<&str>>();

    let winning_numbers = only_numbers
        .first()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();

    let my_numbers = only_numbers
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();

    let num_matches = num_matches(&winning_numbers, &my_numbers);
    println!("card num: {card_number}, card copies: {card_copies}, num_matches: {num_matches}");

    // card 1 has 3 matches
    // +(num of copies of this card) to copies of card 2, 3, and 4
    // which is index 1, 2, and 3

    for i in card_number..(card_number + num_matches) {
        // 1, 2, 3
        if let Some(copies) = card_count.get_mut(i as usize) {
            *copies += card_copies;
        } else {
            card_count.insert(i as usize, card_copies);
        }

        let x = i + 1;
        println!("Adding {card_copies} of card {x}");
        // card_count[i as usize] += card_copies;
    }

    // if num_matches == 0 {
    //     0
    // } else {
    //     2_u32.pow(num_matches - 1)
    // }
}

fn num_matches(winning_numbers: &Vec<u32>, my_numbers: &Vec<u32>) -> u32 {
    winning_numbers
        .iter()
        .map(|num| if my_numbers.contains(num) { 1 } else { 0 })
        .sum()
}
