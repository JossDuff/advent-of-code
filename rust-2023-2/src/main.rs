use std::{fs::read_to_string, u32::MAX};

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let sum: u32 = input.lines().map(|line| obey_the_elf(line)).sum();

    println!("{sum}");
}

// parse into game and return the game number if it's possible
fn obey_the_elf(line: &str) -> u32 {
    let mut input: Vec<&str> = line.split(";").collect();
    let game_header: &str = input
        .first()
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()
        .first()
        .unwrap();

    let game_id: u32 = game_header
        .split_whitespace()
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let mut game = Game::new(game_id);

    // remove game header info from first set
    let header_len = game_header.len() + 1;
    let first_set = &input.first().unwrap()[header_len..];
    input[0] = first_set;

    for set in input {
        let mut cube_set = CubeSet::default();

        let colors_grabs: Vec<&str> = set.split(",").collect();
        for color_grab in colors_grabs {
            let parts: Vec<&str> = color_grab.split_whitespace().collect();

            let color = parts[1];
            let num: u32 = parts[0].parse().unwrap();

            cube_set.add_color(color, num)
        }

        game.cube_sets.push(cube_set);
    }

    let min_bag = game.find_lowest_possible_bag();
    // println!("{:?}", min_set);
    let power = min_bag.power();
    power
}

struct Game {
    id: u32,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn new(id: u32) -> Game {
        Game {
            id,
            cube_sets: vec![],
        }
    }

    // fn is_possible(&self) -> bool {
    //     let max = CubeSet {
    //         green: 13,
    //         red: 12,
    //         blue: 14,
    //     };

    //     for set in &self.cube_sets {
    //         if set.green > max.green || set.red > max.red || set.blue > max.blue {
    //             return false;
    //         }
    //     }

    //     true
    // }

    fn find_lowest_possible_bag(&self) -> CubeSet {
        let initial_max_values = CubeSet {
            green: u32::MIN,
            red: u32::MIN,
            blue: u32::MIN,
        };

        let min_set = self
            .cube_sets
            .iter()
            .fold(initial_max_values, |min_bag, cube_set| CubeSet {
                green: min_bag.green.max(cube_set.green),
                red: min_bag.red.max(cube_set.red),
                blue: min_bag.blue.max(cube_set.blue),
            });

        min_set
    }
}

#[derive(Default, Debug)]
struct CubeSet {
    green: u32,
    red: u32,
    blue: u32,
}

impl CubeSet {
    fn add_color(&mut self, color: &str, num: u32) {
        match color {
            "green" => self.green = num,
            "red" => self.red = num,
            "blue" => self.blue = num,
            _ => panic!("unknown color: {color}"),
        }
    }

    fn power(&self) -> u32 {
        self.green * self.red * self.blue
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_min_1() {
        let line = "Game 1: 6 green, 3 blue; 3 red, 1 green; 4 green, 3 red, 5 blue";
        let power = obey_the_elf(line);

        let expected_power = 1 * 3 * 3;
        assert_eq!(power, expected_power);
    }
}
