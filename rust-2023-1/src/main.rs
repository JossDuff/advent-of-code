use anyhow::Result;
use rayon::iter::ParallelIterator;
use rayon::str::ParallelString;
use std::fs::{self};

fn main() -> Result<()> {
    let valid_numbers: Vec<(&str, &str)> = vec![
        ("one", "1"),
        ("1", "1"),
        ("two", "2"),
        ("2", "2"),
        ("three", "3"),
        ("3", "3"),
        ("four", "4"),
        ("4", "4"),
        ("five", "5"),
        ("5", "5"),
        ("six", "6"),
        ("6", "6"),
        ("seven", "7"),
        ("7", "7"),
        ("eight", "8"),
        ("8", "8"),
        ("nine", "9"),
        ("9", "9"),
    ];

    let input = fs::read_to_string("input.txt").unwrap();

    let sum: u32 = input
        .lines()
        .map(|line| find_num(line, &valid_numbers))
        .sum();

    println!("sum: {}", sum);

    Ok(())
}

fn find_num(line: &str, valid_numbers: &Vec<(&str, &str)>) -> u32 {
    let mut left: (usize, &str) = (1000, "");
    let mut right: (i32, &str) = (-1, "");

    for (num, val) in valid_numbers {
        let left_num = line.find(*num);
        if let Some(index) = left_num {
            if index < left.0 {
                left = (index, *val)
            }
        }

        let right_num = line.rfind(*num);
        if let Some(index) = right_num {
            let index: i32 = index.try_into().unwrap();
            if index > right.0 {
                right = (index, *val)
            }
        }
    }

    let number = format!("{}{}", left.1, right.1);
    number.parse::<u32>().unwrap()
}

fn _part_one() {
    let input = fs::read_to_string("input.txt").unwrap();

    let sum: u32 = input
        .par_lines()
        .map(|line| {
            let first = line.chars().find(|c| c.is_digit(10)).unwrap();
            let rev: String = line.chars().rev().collect();
            let last = rev.chars().find(|c| c.is_digit(10)).unwrap();

            let num = format!("{}{}", first, last);

            num.parse::<u32>().unwrap()
        })
        .sum();

    println!("sum: {}", sum);
}

#[cfg(test)]
mod test {
    use crate::find_num;

    fn valid_numbers() -> Vec<(&'static str, &'static str)> {
        vec![
            ("one", "1"),
            ("1", "1"),
            ("two", "2"),
            ("2", "2"),
            ("three", "3"),
            ("3", "3"),
            ("four", "4"),
            ("4", "4"),
            ("five", "5"),
            ("5", "5"),
            ("six", "6"),
            ("6", "6"),
            ("seven", "7"),
            ("7", "7"),
            ("eight", "8"),
            ("8", "8"),
            ("nine", "9"),
            ("9", "9"),
        ]
    }

    #[test]
    fn test_abconeone() {
        let line = "abconeone";

        let num = find_num(line, &valid_numbers());

        assert_eq!(num, 11);
    }

    #[test]
    fn test_abcone1() {
        let line = "abcone1";

        let num = find_num(line, &valid_numbers());

        assert_eq!(num, 11);
    }

    #[test]
    fn test_120zero00one0() {
        let line = "120zero00one0";

        let num = find_num(line, &valid_numbers());

        assert_eq!(num, 11);
    }

    #[test]
    fn test_twoone215three() {
        let line = "twoone215three";

        let num = find_num(line, &valid_numbers());

        assert_eq!(num, 23);
    }

    #[test]
    fn test_11() {
        let line = "11";

        let num = find_num(line, &valid_numbers());

        assert_eq!(num, 11);
    }

    #[test]
    fn test_042znine() {
        let line = "042znine";

        let num = find_num(line, &valid_numbers());

        assert_eq!(num, 49);
    }

    #[test]
    fn test_qjgcxccgthree85five() {
        let line = "qjgcxccgthree85five";

        let num = find_num(line, &valid_numbers());

        assert_eq!(num, 35);
    }

    #[test]
    fn test_seven331fivekfrqbd() {
        let line = "seven331fivekfrqbd";

        let num = find_num(line, &valid_numbers());

        assert_eq!(num, 75);
    }

    #[test]
    fn test_fourfqxkjct554() {
        let line = "fourfqxkjct554";

        let num = find_num(line, &valid_numbers());

        assert_eq!(num, 44);
    }

    #[test]
    fn test_twonzhmrjlgvtvcj6() {
        let line = "twonzhmrjlgvtvcj6";

        let num = find_num(line, &valid_numbers());

        assert_eq!(num, 26);
    }

    #[test]
    fn test_zone4() {
        let line = "zone4";

        let num = find_num(line, &valid_numbers());

        assert_eq!(num, 14);
    }

    #[test]
    fn test_1() {
        let line = "1";

        let num = find_num(line, &valid_numbers());

        assert_eq!(num, 11);
    }

    #[test]
    fn test_one() {
        let line = "one";

        let num = find_num(line, &valid_numbers());

        assert_eq!(num, 11);
    }
}
