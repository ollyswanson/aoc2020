use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let part_1 = input
        .split("\n\n")
        .fold(0, |acc, group| acc + distinct_yes(group));

    println!("Part 1: {}", part_1);

    let part_2 = input
        .split("\n\n")
        .fold(0, |acc, group| acc + all_yes(group));

    println!("Part 2: {}", part_2);

    Ok(())
}

fn distinct_yes(group: &str) -> usize {
    let mut set = HashSet::new();

    for line in group.lines() {
        for &b in line.as_bytes() {
            set.insert(b);
        }
    }

    set.len()
}

fn all_yes(group: &str) -> usize {
    let mut map = HashMap::new();
    let mut len = 0;

    for (i, line) in group.lines().enumerate() {
        for &b in line.as_bytes() {
            let value = map.entry(b).or_insert(0);
            *value += 1;
            len = i + 1;
        }
    }

    map.values().fold(0, |acc, &count| {
        if count == len {
            return acc + 1;
        }
        acc
    })
}
