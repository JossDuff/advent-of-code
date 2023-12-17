use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let sum: u32 = input.lines().map(|line| determine_possibility(line)).sum();

    println!("{sum}");
}

// parse into game and return the game number if it's possible
fn determine_possibility(line: &str) -> u32 {
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

    if game.is_possible() {
        game.id
    } else {
        0
    }
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

    fn is_possible(&self) -> bool {
        let max = CubeSet {
            green: 13,
            red: 12,
            blue: 14,
        };

        for set in &self.cube_sets {
            if set.green > max.green || set.red > max.red || set.blue > max.blue {
                return false;
            }
        }

        true
    }
}

#[derive(Default)]
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
}
