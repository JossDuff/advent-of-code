mod parse;
mod types;

use parse::parse_input;
use std::{collections::HashMap, fs::read_to_string, ops::Index, str::FromStr};
use types::Field;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let field = parse_input(input);
}
