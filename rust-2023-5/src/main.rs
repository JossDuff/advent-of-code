use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let input_file = "input.txt";
    let (seeds, mappings) = parse_input(input_file).unwrap();
    let answer: u64 = lowest_location(seeds, mappings);
    println!("answer: {answer}");
}

fn lowest_location(seeds: Vec<u64>, mappings: Vec<Vec<Mapping>>) -> u64 {
    let mut locations: Vec<u64> = Vec::new();
    for seed in seeds.iter() {
        let mut source: u64 = *seed;
        for category in mappings.iter() {
            for map in category {
                let destination = map.destination_of(source);
                if let Some(destination) = destination {
                    source = destination;
                    break;
                }
            }
        }
        locations.push(source)
    }
    *locations.iter().min().unwrap()
}

fn parse_input(input_file: &str) -> Result<(Vec<u64>, Vec<Vec<Mapping>>)> {
    let file = File::open(input_file).unwrap();

    let reader = BufReader::new(file);

    // Read the file into a single string
    let content = reader
        .lines()
        .collect::<Result<Vec<String>, _>>()?
        .join("\n");

    // Split the content into chunks by double newline (empty line)
    let chunks: Vec<&str> = content.split("\n\n").collect();

    let mut mappings: Vec<Vec<Mapping>> = Vec::new();
    let mut seeds: Vec<u64> = Vec::new();

    // Iterate through each chunk
    for (i, chunk) in chunks.iter().enumerate() {
        // println!("Chunk {}:", i + 1);
        if i == 0 {
            let nums: Vec<&str> = chunk.split(' ').collect();
            // skip the first "seeds:"
            for num in nums.iter().skip(1) {
                // println!("{num}");
                seeds.push(num.parse::<u64>().unwrap())
            }
            continue;
        }

        let mut mapping_category = Vec::new();

        // Iterate through each line in the chunk
        // Skip the first line in most cases.
        for line in chunk.lines().skip(1) {
            //     println!("{}", line);
            let nums: Vec<&str> = line.split(' ').collect();
            assert!(nums.len() == 3);
            mapping_category.push(Mapping {
                destination_range_start: nums[0].parse::<u64>().unwrap(),
                source_range_start: nums[1].parse::<u64>().unwrap(),
                range_length: nums[2].parse::<u64>().unwrap(),
            })
        }
        mappings.push(mapping_category);

        // println!("----------"); // Separator between chunks
    }
    Ok((seeds, mappings))
}

struct Mapping {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl Mapping {
    fn destination_of(&self, source: u64) -> Option<u64> {
        let source_range_end = self.source_range_start + self.range_length - 1;
        let source_range = (self.source_range_start, source_range_end);
        // the case when source isn't in the range
        if source < source_range.0 || source > source_range.1 {
            None
        } else {
            // source is in the range
            let offset = source - source_range.0;
            let destination = self.destination_range_start + offset;
            Some(destination)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Mapping;

    #[test]
    fn test_destination_mapping() {
        let m = Mapping {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        };

        assert_eq!(m.destination_of(98), Some(50));
        assert_eq!(m.destination_of(99), Some(51));
        assert_eq!(m.destination_of(100), None);
        assert_eq!(m.destination_of(97), None);
    }
}
