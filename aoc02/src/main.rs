use aoc02::Entry;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let entries = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Entry>, _>>()?;

    let valid_entries: u32 = entries.iter().fold(0, |acc, entry| {
        if entry.valid_old() {
            return acc + 1;
        }
        acc
    });

    println!("Part 1: {}", valid_entries);

    let valid_entries: u32 = entries.iter().fold(0, |acc, entry| {
        if entry.valid_new() {
            return acc + 1;
        }
        acc
    });

    println!("Part 2: {}", valid_entries);

    Ok(())
}
