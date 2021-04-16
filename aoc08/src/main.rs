use aoc08::{Executable, Program, Result};
use std::io;

fn main() -> Result<()> {
    let mut executable = Executable::from_reader(&mut io::stdin().lock())?;
    let mut program = Program::new(&mut executable);

    // part 1
    program.execute_from(0);
    println!("Part 1: {}", program.acc);

    program.reset();

    // Part 2
    program.repair_executable()?;
    println!("Part 2: {}", program.acc);

    Ok(())
}
