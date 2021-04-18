use anyhow::{anyhow, Error, Result};
use once_cell::sync::OnceCell;
use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

pub struct Program {
    instructions: Instructions,
    memory: HashMap<u64, u64>,
}

impl Program {
    pub fn new(instructions: Instructions) -> Self {
        let memory = HashMap::new();

        Self {
            instructions,
            memory,
        }
    }

    pub fn run(&mut self) {
        let mut mask = Mask::new();

        for instruction in self.instructions.0.iter() {
            match instruction {
                Instruction::Mask(m) => {
                    mask.update(m);
                }
                Instruction::Mov { addr, val } => {
                    let val = mask.apply_mask(*val);
                    self.memory.insert(*addr, val);
                }
            }
        }
    }

    pub fn sum_values(&self) -> u64 {
        self.memory.values().sum()
    }
}

pub struct Instructions(Vec<Instruction>);

impl Instructions {
    pub fn from_reader<R: BufRead>(reader: R) -> Result<Self> {
        let instructions = reader
            .lines()
            .map(|line| match line {
                Ok(line) => line.trim().parse(),
                Err(e) => Err(Error::from(e)),
            })
            .collect::<Result<_>>()?;

        Ok(Self(instructions))
    }
}

enum Instruction {
    Mov { addr: u64, val: u64 },
    Mask(Vec<u8>),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        static MASK: OnceCell<Regex> = OnceCell::new();
        static MOV: OnceCell<Regex> = OnceCell::new();

        let mask = MASK.get_or_init(|| Regex::new(r"mask\s=\s(?P<mask>[01X]{36})").unwrap());
        let mov = MOV.get_or_init(|| Regex::new(r"mem\[(?P<addr>\d+)\]\s=\s(?P<val>\d+)").unwrap());

        if let Some(mov) = mov.captures(s) {
            return Ok(Self::Mov {
                addr: mov["addr"].parse()?,
                val: mov["val"].parse()?,
            });
        }

        if let Some(mask) = mask.captures(s) {
            return Ok(Self::Mask(mask["mask"].bytes().into_iter().collect()));
        }

        Err(anyhow!("No matches"))
    }
}

/// We can apply the mask X01 (where X means to leave the bit alone) by creating two separate
/// masks. We create an AND mask where X01 becomes 101 and an OR mask where X01 becomes 001. We can
/// see that by applying them sequentially the AND mask will first turn off any 1s that need to
/// become 0s, and then OR mask will turn on anything that needs to be a 1. Both masks will leave
/// the X bits alone.
#[derive(Default)]
struct Mask {
    and: u64,
    or: u64,
}

impl Mask {
    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, s: &[u8]) {
        let (and, or) = s.iter().fold((0, 0), |mut masks, b| {
            let (and, or) = match b {
                b'X' => (1, 0),
                b'0' => (0, 0),
                b'1' => (1, 1),
                _ => panic!("Unrecognized symbol"),
            };
            masks.0 = (masks.0 << 1) | and;
            masks.1 = (masks.1 << 1) | or;

            masks
        });

        self.and = and;
        self.or = or;
    }

    fn apply_mask(&self, mut x: u64) -> u64 {
        x &= self.and;
        x |= self.or;
        x
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn part_1() {
        let instructions = "\
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            mem[8] = 11
            mem[7] = 101
            mem[8] = 0\
        ";
        let instructions = Cursor::new(instructions);
        let instructions = Instructions::from_reader(instructions).unwrap();
        let mut program = Program::new(instructions);

        program.run();
        assert_eq!(program.sum_values(), 165);
    }
}
