use once_cell::sync::OnceCell;
use regex::Regex;
use std::str::FromStr;

pub struct Entry {
    lower: u32,
    upper: u32,
    letter: u8,
    password: Vec<u8>,
}

impl Entry {
    pub fn valid_old(&self) -> bool {
        let count = self.password.iter().fold(0, |acc, &cur| {
            if cur == self.letter {
                return acc + 1;
            }
            acc
        });

        count >= self.lower && count <= self.upper
    }

    pub fn valid_new(&self) -> bool {
        if self.password.len() < self.upper as usize {
            return false;
        }

        let mut matches = 0;

        if self.password[self.lower as usize - 1] == self.letter {
            matches += 1;
        }

        if self.password[self.upper as usize - 1] == self.letter {
            matches += 1;
        }

        matches == 1
    }
}

impl FromStr for Entry {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 1-3 a: abcde
        // 1-3 b: cdefg
        // 2-9 c: ccccccccc
        static RE: OnceCell<Regex> = OnceCell::new();
        let regex = RE
            .get_or_try_init(|| {
                Regex::new(
                    r"(?x)
                    (?P<lower>[0-9]+)-(?P<upper>[0-9]+)
                    \s+
                    (?P<character>[a-z]):\s+
                    (?P<password>[a-z]+)
                    ",
                )
            })
            .unwrap();

        let caps = match regex.captures(s) {
            Some(caps) => caps,
            None => {
                return Err("unrecognized entry".into());
            }
        };

        let password: Vec<u8> = caps["password"].bytes().collect();

        Ok(Self {
            lower: caps["lower"].parse()?,
            upper: caps["upper"].parse()?,
            letter: caps["character"].bytes().next().unwrap(),
            password,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let entry_1 = "1-3 a: abcde";
        let entry_2 = "1-3 b: cdefg";
        let entry_3 = "2-9 c: ccccccccc";

        let entry_1: Entry = entry_1.parse().unwrap();
        let entry_2: Entry = entry_2.parse().unwrap();
        let entry_3: Entry = entry_3.parse().unwrap();

        assert!(entry_1.valid_old());
        assert!(!entry_2.valid_old());
        assert!(entry_3.valid_old());
    }

    #[test]
    fn part_2() {
        let entry_1 = "1-3 a: abcde";
        let entry_2 = "1-3 b: cdefg";
        let entry_3 = "2-9 c: ccccccccc";

        let entry_1: Entry = entry_1.parse().unwrap();
        let entry_2: Entry = entry_2.parse().unwrap();
        let entry_3: Entry = entry_3.parse().unwrap();

        assert!(entry_1.valid_new());
        assert!(!entry_2.valid_new());
        assert!(!entry_3.valid_new());
    }
}
