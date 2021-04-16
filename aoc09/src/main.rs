use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input: Vec<i64> = input
        .lines()
        .map(|line| line.trim().parse())
        .collect::<Result<_, _>>()?;

    let target = part_1(&input, 25).ok_or("Couldn't answer part_1")?;
    println!("{}", target);

    let sum = part_2(&input, target).ok_or("Couldn't answer part_2")?;
    println!("{}", sum);

    Ok(())
}

fn part_1(list: &[i64], window_size: usize) -> Option<i64> {
    let mut set = HashSet::with_capacity(window_size);

    for i in window_size..list.len() {
        set.clear();
        let target = list[i];

        for &a in list[i - window_size..i].iter() {
            set.insert(target - a);
        }

        if list[i - window_size..i].iter().fold(0, |acc, cur| {
            if set.get(cur).is_some() && *cur != target - cur {
                return acc + 1;
            }
            acc
        }) == 0
        {
            return Some(target);
        };
    }

    None
}

fn part_2(list: &[i64], target: i64) -> Option<i64> {
    let prefix_sum: Vec<_> = list
        .iter()
        .scan(0, |state, cur| {
            *state += *cur;
            Some(*state)
        })
        .collect();

    // Sliding windows
    for window_size in 2..list.len() - 1 {
        for i in 1..list.len() - window_size {
            if prefix_sum[i + window_size] - prefix_sum[i - 1] == target {
                let slice = &list[i..=i + window_size];
                return Some(slice.iter().max().unwrap() + slice.iter().min().unwrap());
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let list = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(part_1(&list, 5), Some(127));
    }

    #[test]
    fn test_2() {
        let list = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(part_2(&list, 127), Some(62));
    }
}
