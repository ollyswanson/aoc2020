use anyhow::Result;
use aoc14::{Instructions, Program};
use std::io;

fn main() -> Result<()> {
    let instructions = Instructions::from_reader(io::stdin().lock())?;
    let mut program = Program::new(instructions);
    let mut program_2 = program.clone();

    program.run();
    println!("Part 1: {}", program.sum_values());

    program_2.run_alt();
    println!("Part 2: {}", program_2.sum_values());

    Ok(())
}
