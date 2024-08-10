mod direction;

use direction::Direction;

use num::{integer::lcm, Integer};
use std::{collections::HashMap, fs::read_to_string, ops::Index, str::FromStr};

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let (starting_nodes, nodes, directions) = parse_input(input);

    println!(
        "{} starting_nodes, {} nodes, {} directions",
        starting_nodes.len(),
        nodes.len(),
        directions.len()
    );

    let steps = traverse(starting_nodes, nodes, directions);

    println!("steps: {steps}");
}

fn traverse(
    starting_nodes: Vec<String>,
    nodes: HashMap<String, Node>,
    directions: Vec<Direction>,
) -> usize {
    let steps: Vec<usize> = starting_nodes
        .iter()
        .map(|n| find_steps_from(n, &nodes, &directions))
        .collect();

    steps.into_iter().reduce(|acc, s| acc.lcm(&s)).unwrap()
}

fn find_steps_from(
    starting_node: &str,
    nodes: &HashMap<String, Node>,
    directions: &[Direction],
) -> usize {
    let mut current_node = starting_node;

    let mut steps = 0;
    loop {
        if current_node.ends_with("Z") {
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

fn parse_input(input: String) -> (Vec<String>, HashMap<String, Node>, Vec<Direction>) {
    let split: Vec<&str> = input.split("\n\n").collect();
    assert!(split.len() == 2);

    let directions = parse_directions(split[0]);
    let (starting_nodes, nodes) = parse_nodes(split[1]);

    (starting_nodes, nodes, directions)
}

fn parse_nodes(input: &str) -> (Vec<String>, HashMap<String, Node>) {
    let mut map: HashMap<String, Node> = HashMap::new();
    let mut starting_nodes = Vec::new();

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

        if split[0].ends_with("A") {
            starting_nodes.push(split[0].into());
        }
    });

    (starting_nodes, map)
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input
        .chars()
        .map(|s| Direction::from_str(&s.to_string()).unwrap())
        .collect()
}

struct Node {
    left: String,
    right: String,
}
