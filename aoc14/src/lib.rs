use anyhow::{anyhow, Error, Result};
use itertools::Itertools;
use once_cell::sync::OnceCell;
use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Clone)]
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

    pub fn run_alt(&mut self) {
        let mut mask = Mask::new();

        for instruction in self.instructions.0.iter() {
            match instruction {
                Instruction::Mask(m) => {
                    mask.update(m);
                }
                Instruction::Mov { addr, val } => {
                    for addr in mask.memory_addresses(*addr) {
                        self.memory.insert(addr, *val);
                    }
                }
            }
        }
    }

    pub fn sum_values(&self) -> u64 {
        self.memory.values().sum()
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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

    fn apply_mask(&self, x: u64) -> u64 {
        x & self.and | self.or
    }

    fn floating_positions(&self) -> impl Iterator<Item = u64> + '_ {
        // mask where all the 1s are where the Xs are in the mask and 0s elsewhere
        let x = self.and & (!self.or);
        (0u64..36).filter(move |&i| (1 << i) & x != 0)
    }

    fn memory_addresses(&self, addr: u64) -> impl Iterator<Item = u64> + '_ {
        let addr = addr | self.or;
        // turn off Xes
        let addr = self
            .floating_positions()
            .fold(addr, |addr, pos| !(1 << pos) & addr);

        // return iterator that turns them back on to produce all of the different floating
        // combinations
        self.floating_positions()
            .powerset()
            .map(move |x| self.apply_x(addr, &x))
    }

    fn apply_x(&self, addr: u64, positions: &[u64]) -> u64 {
        positions
            .iter()
            .fold(addr, |addr, position| (1 << position) | addr)
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

    #[test]
    fn part_2() {
        let instructions = "\
            mask = 000000000000000000000000000000X1001X
            mem[42] = 100
            mask = 00000000000000000000000000000000X0XX
            mem[26] = 1\
        ";
        let instructions = Cursor::new(instructions);
        let instructions = Instructions::from_reader(instructions).unwrap();
        let mut program = Program::new(instructions);

        program.run_alt();
        assert_eq!(program.sum_values(), 208);
    }
}
