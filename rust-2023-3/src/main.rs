use anyhow::Result;
use std::{cmp::min, fs::read_to_string};

fn main() {
    // let symbols = vec!['@', '#', '$', '%', '&', '*', '-', '+', '=', '/'];
    let input = read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut symbol_locations: Vec<SymbolLocation> = lines
        .iter()
        .enumerate()
        .flat_map(|(line_num, line)| {
            line.char_indices()
                .filter(|(_, c)| *c == '*')
                .map(|(index, _)| SymbolLocation {
                    line: line_num,
                    index,
                    nums: vec![],
                })
                .collect::<Vec<SymbolLocation>>()
        })
        .collect();

    let mut sum: u32 = 0;
    for location in &mut symbol_locations {
        check_above_and_below(location, &lines);
        check_adjacent(location, &lines);
        // println!("{location:?}")
    }

    let sum: u32 = symbol_locations
        .iter()
        .map(|location| {
            if location.nums.len() == 2 {
                location.nums[0] * location.nums[1]
            } else {
                0
            }
        })
        .sum();

    println!("{sum}");
}

// will be tricky
fn check_above_and_below(location: &mut SymbolLocation, lines: &Vec<&str>) {
    // check above line
    if let Some(line) = lines.get(location.line + 1) {
        check_above_and_below_impl(location, line);
    };

    // check below line
    if let Some(line) = lines.get(location.line - 1) {
        check_above_and_below_impl(location, line);
    };
}

fn check_above_and_below_impl(location: &mut SymbolLocation, line: &str) {
    let dist_left = min(3, location.index);
    let dist_right = min(4, line.len() - location.index);
    let selection = &line[location.index - dist_left..location.index + dist_right];
    // print!("{selection} | ");
    sum_above_or_below_selection(selection, location);
    // print!("{sum}\n");
}

fn sum_above_or_below_selection(selection: &str, location: &mut SymbolLocation) {
    let mut curr_num = String::new();
    for (idx, c) in selection.chars().enumerate() {
        match c {
            '0'..='9' => {
                if !(curr_num.is_empty() && idx > 4) {
                    curr_num.push(c)
                }
            }
            _ => {
                if !curr_num.is_empty() && idx != 1 && idx != 2 {
                    location.nums.push(curr_num.parse::<u32>().unwrap());
                }

                curr_num.clear()
            }
        }
    }

    if !curr_num.is_empty() && curr_num.len() == 3 {
        location.nums.push(curr_num.parse::<u32>().unwrap());
    }
}

fn check_adjacent(location: &mut SymbolLocation, lines: &Vec<&str>) {
    // get line
    let line = lines[location.line];
    // get slice of index
    let dist_left = min(3, location.index);
    let left = line.get(location.index - dist_left..location.index);

    let num_left = if let Some(str) = left {
        // print!("{str}");
        match str.parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                let str = &str[1..];
                match str.parse::<u32>() {
                    Ok(num) => num,
                    Err(_) => {
                        let str = &str[1..];
                        match str.parse::<u32>() {
                            Ok(num) => num,
                            Err(_) => 0,
                        }
                    }
                }
            }
        }
    } else {
        0
    };
    // print!(" {num}");

    let dist_right = min(4, line.len() - location.index);
    let right = line.get(location.index + 1..location.index + dist_right);
    let num_right = if let Some(str) = right {
        // print!(" | {str}");
        match str.parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                let str = &str[..2];
                match str.parse::<u32>() {
                    Ok(num) => num,
                    Err(_) => {
                        let str = &str[..1];
                        match str.parse::<u32>() {
                            Ok(num) => num,
                            Err(_) => 0,
                        }
                    }
                }
            }
        }
    } else {
        0
    };

    // print!(" {num} \n");

    if num_left > 0 {
        location.nums.push(num_left)
    }

    if num_right > 0 {
        location.nums.push(num_right)
    }
}

#[derive(Debug)]
struct SymbolLocation {
    line: usize,
    index: usize,
    nums: Vec<u32>,
}

/*

Tests were for part 1

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sum_above_or_below_1() {
        let line = "7...403";
        let sum = sum_above_or_below_selection(line);
        let expected_sum: u32 = 403;
        assert_eq!(sum, expected_sum)
    }

    #[test]
    fn sum_above_or_below_2() {
        let line = "7....03";
        let sum = sum_above_or_below_selection(line);
        let expected_sum: u32 = 0;
        assert_eq!(sum, expected_sum)
    }

    #[test]
    fn sum_above_or_below_3() {
        let line = "70...03";
        let sum = sum_above_or_below_selection(line);
        let expected_sum: u32 = 0;
        assert_eq!(sum, expected_sum)
    }

    #[test]
    fn sum_above_or_below_4() {
        let line = "7.44.03";
        let sum = sum_above_or_below_selection(line);
        let expected_sum: u32 = 44;
        assert_eq!(sum, expected_sum)
    }

    #[test]
    fn sum_above_or_below_5() {
        let line = "7.1#103";
        let sum = sum_above_or_below_selection(line);
        let expected_sum: u32 = 104;
        assert_eq!(sum, expected_sum)
    }

    #[test]
    fn sum_above_or_below_6() {
        let line = "100.103";
        let sum = sum_above_or_below_selection(line);
        let expected_sum: u32 = 203;
        assert_eq!(sum, expected_sum)
    }

    #[test]
    fn sum_above_or_below_7() {
        let line = "700&.03";
        let sum = sum_above_or_below_selection(line);
        let expected_sum: u32 = 700;
        assert_eq!(sum, expected_sum)
    }

    #[test]
    fn sum_above_or_below_8() {
        let line = "7...*03";
        let sum = sum_above_or_below_selection(line);
        let expected_sum: u32 = 0;
        assert_eq!(sum, expected_sum)
    }

    #[test]
    fn sum_above_or_below_9() {
        let line = "7...33&";
        let sum = sum_above_or_below_selection(line);
        let expected_sum: u32 = 33;
        assert_eq!(sum, expected_sum)
    }

    #[test]
    fn sum_above_or_below_10() {
        let line = ".&7..3&";
        let sum = sum_above_or_below_selection(line);
        let expected_sum: u32 = 7;
        assert_eq!(sum, expected_sum)
    }
}
*/
