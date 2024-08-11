use crate::types::{Field, Pipe};
use std::str::FromStr;

pub fn parse_input(input: String) -> Field {
    Field(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Pipe::from_str(&c.to_string()).unwrap())
                    .collect::<Vec<Pipe>>()
            })
            .collect::<Vec<Vec<Pipe>>>(),
    )
}
