use anyhow::Result;
use aoc14::{Instructions, Program};
use std::io;

fn main() -> Result<()> {
    let instructions = Instructions::from_reader(io::stdin().lock())?;
    let mut program = Program::new(instructions);

    program.run();
    println!("Part 1: {}", program.sum_values());

    Ok(())
}
