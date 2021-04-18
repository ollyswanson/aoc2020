use anyhow::{anyhow, Result};
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut input = input.lines();

    let arrival: i64 = input.next().ok_or(anyhow!("Invalid input"))?.parse()?;

    let line_2 = input.next().ok_or(anyhow!("Invalid input"))?;
    let bus_ids: Vec<i64> = line_2.split(",").filter_map(|s| s.parse().ok()).collect();

    let first_bus = part_1(&bus_ids, arrival).ok_or(anyhow!("No buses!"))?;
    println!("Part 1: {}", first_bus.0 * first_bus.1);

    let departure_pattern: Vec<(i64, i64)> = line_2
        .split(",")
        .enumerate()
        .filter_map(|(i, s)| match s.parse() {
            Ok(id) => Some((i as i64, id)),
            Err(_) => None,
        })
        .collect();

    let timestamp = part_2(&departure_pattern);
    println!("Part 2: {}", timestamp);

    Ok(())
}

/// Returns the first interval that repeats after the intersection and how long after the
/// intersection it repeated
fn part_1(intervals: &[i64], intersect: i64) -> Option<(i64, i64)> {
    let repeats = intervals
        .iter()
        .copied()
        .map(|interval| {
            let gap = interval - intersect % interval;
            (interval, gap)
        })
        .min_by(|x, y| x.1.cmp(&y.1));

    repeats
}

// Part 2
/// Part 2 can be solved by applying the chinese remainder theorem to the pattern of buses and
/// their ids (period). We are essentially trying to solve the system of congruences where
/// timestamp == i mod id, where i is the position in the pattern and id is the bus id / period.
/// The input is such that the position in the pattern and the id are coprime.
fn part_2(pattern: &[(i64, i64)]) -> i64 {
    let product = pattern.iter().fold(1, |acc, cur| acc * cur.1);

    let timestamp = pattern.iter().fold(0, |acc, cur| {
        let i = cur.0;
        let period = cur.1;
        let coeff = product / period;

        acc + coeff * i * mod_inverse(coeff, period)
    });

    (timestamp % product).abs()
}

/// returns gcd: q and 2 coefficients x0, y0 such that x * x0 + y * y0 = gcd(x, y)
fn extended_euclid(mut x: i64, mut y: i64) -> (i64, i64, i64) {
    let (mut x0, mut x1, mut y0, mut y1) = (1, 0, 0, 1);

    while y > 0 {
        let tmp = x;
        let q = x / y;
        x = y;
        y = tmp % y;

        let tmp = x0;
        x0 = x1;
        x1 = tmp - q * x1;

        let tmp = y0;
        y0 = y1;
        y1 = tmp - q * y1;
    }

    (x, x0, y0)
}

fn mod_inverse(a: i64, m: i64) -> i64 {
    let (g, x, _) = extended_euclid(a, m);

    if g != 1 {
        panic!(format!("{} & {} are not coprime", a, m));
    }

    x % m
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_2() {
        let departure_pattern = "67,7,x,59,61";
        let departure_pattern: Vec<(i64, i64)> = departure_pattern
            .split(",")
            .enumerate()
            .filter_map(|(i, s)| match s.parse() {
                Ok(id) => Some((i as i64, id)),
                Err(_) => None,
            })
            .collect();

        assert_eq!(part_2(&departure_pattern), 1261476);
    }
}
