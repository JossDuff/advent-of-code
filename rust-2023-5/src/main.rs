use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    // let input = read_to_string("input.txt").unwrap();

    // let input = input.lines().map(|line| parse_line(line, &mut card_count));
    let input_file = "test-input.txt";
    let _ = parse_input(input_file).unwrap();
}

fn parse_input(input_file: &str) -> Result<()> {
    let file = File::open(input_file).unwrap();

    let reader = BufReader::new(file);

    // Read the file into a single string
    let content = reader
        .lines()
        .collect::<Result<Vec<String>, _>>()?
        .join("\n");

    // Split the content into chunks by double newline (empty line)
    let chunks: Vec<&str> = content.split("\n\n").collect();

    // Iterate through each chunk
    for (i, chunk) in chunks.iter().enumerate() {
        println!("Chunk {}:", i + 1);

        // Iterate through each line in the chunk
        for line in chunk.lines() {
            println!("{}", line);
        }

        println!("----------"); // Separator between chunks
    }
    Ok(())
}

// fn parse_line(line: &str, card_count: &mut Vec<u32>) {
//     let initial_split = line.split(":").collect::<Vec<&str>>();
//
//     let card_number: u32 = if let Some(x) = initial_split.first().unwrap().split_whitespace().last()
//     {
//         x.parse().unwrap()
//     } else {
//         return;
//     };
// }
