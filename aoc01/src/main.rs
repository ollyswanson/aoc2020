use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    // TODO: Find out why it's not working with Box<dyn Error>
    let input = input
        .lines()
        .map(|line| line.trim().parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;

    if let Some(answer) = part_1(&input, 2020) {
        println!("Part 1: {}", answer);
    } else {
        return Err("Didn't find an answer for part_1".into());
    }

    if let Some(answer) = part_2(&input, 2020) {
        println!("Part 2: {}", answer);
    } else {
        return Err("Didn't find an answer for part_1".into());
    }

    Ok(())
}

// O(n)
fn part_1(input: &[i32], target: i32) -> Option<i32> {
    let map: HashMap<_, _> = input.iter().map(|&i| (target - i, i)).collect();

    for &i in input.iter() {
        if let Some(&j) = map.get(&i) {
            return Some(i * j);
        }
    }

    None
}

// O(n^2) (Not sure if you can do better than that).
fn part_2(input: &[i32], target: i32) -> Option<i32> {
    // learn to use iter tools so that you can do this with iterators
    let len = input.len();
    let mut map: HashMap<i32, (i32, i32)> = HashMap::with_capacity(len * (len - 1));

    for (i, &a) in input.iter().enumerate() {
        for j in i + 1..len {
            let b = input[j];
            let key = target - a - b;
            map.insert(key, (a, b));
        }
    }

    for &a in input.iter() {
        if let Some(&(b, c)) = map.get(&a) {
            return Some(a * b * c);
        }
    }

    None
}
