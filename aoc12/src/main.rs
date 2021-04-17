use aoc12::{AocError, FirstBoat, Instructions, SecondBoat};
use std::io;

fn main() -> Result<(), AocError> {
    let instructions = Instructions::from_reader(io::stdin().lock())?;
    let mut first_boat = FirstBoat::new();

    first_boat.follow_instructions(&instructions);
    println!("Part 1: {}", first_boat.manhattan_distance());

    let mut second_boat = SecondBoat::new();
    second_boat.follow_instructions(&instructions);
    println!("Part 2: {}", second_boat.manhattan_distance());

    Ok(())
}
