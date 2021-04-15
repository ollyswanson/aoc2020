use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.trim().bytes().collect())
        .collect();

    let part_1 = trees_encountered(&input, 3, 1);

    println!("Part 1: {}", part_1);

    let mut part_2 = 1;
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for (right, down) in slopes {
        part_2 *= trees_encountered(&input, right, down);
    }

    println!("Part 2: {}", part_2);

    Ok(())
}

fn trees_encountered(input: &Vec<Vec<u8>>, right: usize, down: usize) -> u32 {
    let width = input[0].len();
    let mut x = 0;
    let mut trees = 0;

    for (y, row) in input.iter().enumerate() {
        if y % down == 0 && row[x] == b'#' {
            trees += 1;
        }

        x = (x + right) % width;
    }

    trees
}
