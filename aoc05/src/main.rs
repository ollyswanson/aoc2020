use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut ids: Vec<_> = input.lines().map(|line| find_seat_id(line)).collect();

    println!("Part 1: {}", ids.iter().max().unwrap());
    println!("Part 2: {}", find_my_seat(&mut ids).unwrap());

    Ok(())
}

fn find_seat_id(s: &str) -> u32 {
    let mut lower = 0;
    let mut upper = 127;

    for i in 0..6 {
        if s.bytes().nth(i) == Some(b'F') {
            upper = (lower + upper) / 2;
        } else {
            lower = (lower + upper) / 2 + 1;
        }
    }

    let row = if s.bytes().nth(6) == Some(b'F') {
        lower
    } else {
        upper
    };

    let mut left = 0;
    let mut right = 7;

    for i in 7..9 {
        if s.bytes().nth(i) == Some(b'L') {
            right = (left + right) / 2;
        } else {
            left = (left + right) / 2 + 1;
        }
    }

    let column = if s.bytes().nth(9) == Some(b'L') {
        left
    } else {
        right
    };

    row * 8 + column
}

fn find_my_seat(ids: &mut [u32]) -> Option<u32> {
    ids.sort();

    for i in 1..ids.len() - 1 {
        if ids[i] == ids[i + 1] - 2 {
            return Some(ids[i] + 1);
        }
    }

    None
}
