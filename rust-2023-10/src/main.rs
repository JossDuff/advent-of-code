mod parse;
mod types;

use parse::parse_input;
use std::{collections::HashMap, fs::read_to_string, ops::Index, str::FromStr};
use types::{Coord, Field, Pipe};

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let field = parse_input(input);
    let animal_location = field.find_the_animal();

    let (location_a, location_b) = get_starting_locations(&field, animal_location);
    println!("location_a {location_b:?}, location_b {location_b:?}");

    let (mut curr_loc_a, mut curr_loc_b) = (location_a, location_b);
    let (mut prev_loc_a, mut prev_loc_b) = (animal_location, animal_location);
    let mut steps: usize = 0;
    loop {
        if curr_loc_a == curr_loc_b {
            break;
        }

        let buff = curr_loc_a;
        curr_loc_a = field.next(prev_loc_a, curr_loc_a);
        prev_loc_a = buff;

        let buff = curr_loc_b;
        curr_loc_b = field.next(prev_loc_b, prev_loc_b);
        prev_loc_b = buff;

        steps += 1;
    }

    println!("met up in {steps} steps");
}

// gives the two starting coords for the first steps
// Starting out is a little manuel with this approach
fn get_starting_locations(field: &Field, animal_location: Coord) -> (Coord, Coord) {
    let mut valid_directions: Vec<Coord> = Vec::new();

    let north = animal_location.north();
    match field.at(&north) {
        Pipe::Vertical | Pipe::SouthWest7 | Pipe::SouthEastF => {
            println!("north is valid!");
            valid_directions.push(north)
        }
        _ => (),
    };

    let east = animal_location.east();
    match field.at(&east) {
        Pipe::Horizontal | Pipe::NorthWestJ | Pipe::SouthWest7 => {
            println!("east is valid!");
            valid_directions.push(east)
        }
        _ => (),
    }

    let south = animal_location.south();
    match field.at(&south) {
        Pipe::Vertical | Pipe::NorthEastL | Pipe::NorthWestJ => {
            println!("south is valid!");
            valid_directions.push(south)
        }
        _ => (),
    }

    let west = animal_location.west();
    match field.at(&west) {
        Pipe::Horizontal | Pipe::NorthEastL | Pipe::SouthEastF => {
            println!("west is valid!");
            valid_directions.push(west)
        }
        _ => (),
    }

    assert!(valid_directions.len() == 2);

    (valid_directions[0], valid_directions[1])
}
