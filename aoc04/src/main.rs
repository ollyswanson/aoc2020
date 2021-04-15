use regex::Regex;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input: Vec<_> = input.split("\n\n").collect();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    Ok(())
}

// byr (Birth Year)
// iyr (Issue Year)
// eyr (Expiration Year)
// hgt (Height)
// hcl (Hair Color)
// ecl (Eye Color)
// pid (Passport ID)
// cid (Country ID)

fn part_1(input: &[&str]) -> u32 {
    let keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    // assume no repeat fields
    input.iter().fold(0, |acc, passport| {
        if keys.iter().all(|key| {
            passport
                .split_ascii_whitespace()
                .any(|pair| pair.starts_with(key))
        }) {
            return acc + 1;
        }
        acc
    })
}

fn part_2(input: &[&str]) -> u32 {
    let expressions = vec![
        Regex::new(r"\bbyr:(19[2-9]\d|200[0-2])\b").unwrap(),
        Regex::new(r"\biyr:20(1\d|20)\b").unwrap(),
        Regex::new(r"\beyr:20(2\d|30)\b").unwrap(),
        Regex::new(r"\bhgt:(1([5-8]\d|9[0-3])cm|(59|6\d|7[0-6])in)\b").unwrap(),
        Regex::new(r"\bhcl:#[0-9a-f]{6}\b").unwrap(),
        Regex::new(r"\becl:(amb|blu|brn|grn|gry|hzl|oth)\b").unwrap(),
        Regex::new(r"\bpid:\d{9}\b").unwrap(),
    ];

    input.iter().fold(0, |acc, passport| {
        if expressions.iter().all(|exp| exp.is_match(passport)) {
            return acc + 1;
        }
        acc
    })
}
