use std::collections::HashMap;
use std::io::{self, BufRead};
use thiserror::Error;

#[derive(Error, Debug)]
enum AocError {
    #[error("Read stdin error")]
    Stdin(#[from] std::io::Error),
    #[error("Parse int error")]
    ParseErr(#[from] std::num::ParseIntError),
}

fn main() -> Result<(), AocError> {
    let mut input: Vec<u32> = io::stdin()
        .lock()
        .lines()
        .map(|line| match line {
            Ok(line) => line.trim().parse().map_err(|e| AocError::from(e)),
            Err(e) => Err(AocError::from(e)),
        })
        .collect::<Result<_, AocError>>()?;

    input.sort();
    let diff = input.iter().fold(Differences::new(), |mut diff, adapter| {
        match adapter - diff.joltage {
            1 => diff.diff_1 += 1,
            3 => diff.diff_3 += 1,
            _ => {}
        }

        diff.joltage = *adapter;
        diff
    });

    // add 1 for the connection between the final adapter and the appliance
    let part_1 = diff.diff_1 * (diff.diff_3 + 1);
    println!("Part 1: {}", part_1);

    // We need to add the first joltage (0) and the final joltage (max_joltage + 3) to the input
    // for this approach to work.
    let &max = input.last().unwrap();
    input.push(0);
    input.push(max + 3);
    input.sort();

    let mut memoizer = Memoizer::default();
    let combinations = memoizer.count_combinations(&input);
    println!("Part 2: {}", combinations);

    Ok(())
}

// part 1
#[derive(Default)]
struct Differences {
    diff_1: u32,
    diff_3: u32,
    joltage: u32,
}

impl Differences {
    fn new() -> Self {
        Self::default()
    }
}

// part 2, assume sorted input
#[derive(Default)]
struct Memoizer {
    inner: HashMap<u32, u64>,
}

impl Memoizer {
    fn count_combinations(&mut self, adapters: &[u32]) -> u64 {
        if adapters.len() == 1 {
            return 1;
        }

        let first_joltage = adapters[0];
        if let Some(&combinations) = self.inner.get(&first_joltage) {
            return combinations;
        }

        let mut combinations = 0;
        for (i, joltage) in adapters[1..].iter().enumerate() {
            if joltage - first_joltage > 3 {
                break;
            }

            combinations += self.count_combinations(&adapters[i + 1..]);
        }

        self.inner.insert(first_joltage, combinations);

        combinations
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_2() {
        let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        input.sort();

        let mut memoizer = Memoizer::default();
        let combinations = memoizer.count_combinations(&input);

        assert_eq!(combinations, 8);
    }

    #[test]
    fn part_2_alt() {
        let mut input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        input.push(0);
        input.push(52);
        input.sort();

        let mut memoizer = Memoizer::default();
        let combinations = memoizer.count_combinations(&input);

        assert_eq!(combinations, 19208);
    }
}
