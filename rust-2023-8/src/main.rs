mod direction;

use direction::Direction;

use std::{collections::HashMap, fs::read_to_string, str::FromStr};

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let (nodes, directions) = parse_input(input);

    println!("{} nodes and {} directions", nodes.len(), directions.len());
    let steps = traverse(nodes, directions);

    println!("steps: {steps}");
}

struct Node {
    left: String,
    right: String,
}

fn traverse(nodes: HashMap<String, Node>, directions: Vec<Direction>) -> usize {
    let mut current_node = "AAA";
    let mut steps = 0;
    loop {
        if current_node == "ZZZ" {
            break;
        }

        let next_direction = &directions[steps % directions.len()];
        let node = nodes.get(current_node).unwrap();
        current_node = match next_direction {
            Direction::Right => &node.right,
            Direction::Left => &node.left,
        };
        steps += 1;
    }

    steps
}

fn parse_input(input: String) -> (HashMap<String, Node>, Vec<Direction>) {
    let split: Vec<&str> = input.split("\n\n").collect();
    assert!(split.len() == 2);

    let directions = parse_directions(split[0]);
    let nodes = parse_nodes(split[1]);

    (nodes, directions)
}

fn parse_nodes(input: &str) -> HashMap<String, Node> {
    let mut map: HashMap<String, Node> = HashMap::new();

    input.lines().for_each(|line| {
        // parse each line and insert it into the hash map
        let clean_line = line
            .replace("=", "")
            .replace("(", "")
            .replace(")", "")
            .replace(",", "");
        let split: Vec<&str> = clean_line.split(" ").collect();

        map.insert(
            split[0].into(),
            Node {
                left: split[2].into(),
                right: split[3].into(),
            },
        );
    });

    map
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input
        .chars()
        .map(|s| Direction::from_str(&s.to_string()).unwrap())
        .collect()
}
