use aoc11::{AocError, Seating};
use std::io;

fn main() -> Result<(), AocError> {
    let mut seating = Seating::from_reader(io::stdin().lock())?;
    let mut seating_2 = seating.clone();

    seating.process_adjacent();
    println!("Part 1: {}", seating.num_seats());

    seating_2.process_los();
    println!("Part 2: {}", seating_2.num_seats());

    Ok(())
}
