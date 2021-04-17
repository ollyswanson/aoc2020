use std::io::BufRead;
use std::mem;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("Invalid input")]
    InvalidInput,
    #[error("Io error")]
    IoError(#[from] std::io::Error),
}

// Part 1
pub struct FirstBoat {
    /// 4 valid bearings, 0, 90, 180, 270, corresponding to the 4 cardinal directions clockwise
    /// north
    bearing: i32,
    x: i32,
    y: i32,
}

impl FirstBoat {
    pub fn new() -> Self {
        Self {
            bearing: 90,
            x: 0,
            y: 0,
        }
    }

    pub fn follow_instructions(&mut self, instructions: &Instructions) {
        for instruction in instructions.0.iter() {
            match instruction {
                Instruction::Turn(t) => {
                    self.calculate_bearing(t);
                }
                Instruction::Cardinal(c) => {
                    self.cardinal(c);
                }
                Instruction::Forward(dis) => {
                    self.travel(*dis);
                }
            }
        }
    }

    fn calculate_bearing(&mut self, turn: &Turn) {
        match turn {
            Turn::Left(deg) => self.bearing = (self.bearing - deg).rem_euclid(360),
            Turn::Right(deg) => self.bearing = (self.bearing + deg).rem_euclid(360),
        }
    }

    fn travel(&mut self, distance: i32) {
        match self.bearing {
            0 => self.y += distance,
            90 => self.x += distance,
            180 => self.y -= distance,
            270 => self.x -= distance,
            b => panic!(format!("Invalid bearing {}", b)),
        }
    }

    fn cardinal(&mut self, cardinal: &Cardinal) {
        use Cardinal::*;

        match cardinal {
            North(distance) => {
                self.y += *distance;
            }
            East(distance) => {
                self.x += *distance;
            }
            South(distance) => {
                self.y -= *distance;
            }
            West(distance) => {
                self.x -= *distance;
            }
        }
    }

    pub fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

// Part 2
pub struct SecondBoat {
    waypoint_x: i32,
    waypoint_y: i32,
    x: i32,
    y: i32,
}

impl SecondBoat {
    pub fn new() -> Self {
        Self {
            waypoint_x: 10,
            waypoint_y: 1,
            x: 0,
            y: 0,
        }
    }

    pub fn follow_instructions(&mut self, instructions: &Instructions) {
        for instruction in instructions.0.iter() {
            match instruction {
                Instruction::Turn(t) => {
                    self.rotate_waypoint(t);
                }
                Instruction::Cardinal(c) => {
                    self.move_waypoint(c);
                }
                Instruction::Forward(multiplier) => {
                    self.travel(*multiplier);
                }
            }
        }
    }

    fn rotate_waypoint(&mut self, turn: &Turn) {
        // x = x * cos(theta) - y * sin(theta)
        // y = x * sin(theta) + y * cos(theta)
        match turn {
            Turn::Left(90) | Turn::Right(270) => {
                let temp_x = self.waypoint_x;
                self.waypoint_x = -self.waypoint_y;
                self.waypoint_y = temp_x
            }
            Turn::Left(180) | Turn::Right(180) => {
                self.waypoint_x = -self.waypoint_x;
                self.waypoint_y = -self.waypoint_y;
            }
            Turn::Left(270) | Turn::Right(90) => {
                let temp_x = self.waypoint_x;
                self.waypoint_x = self.waypoint_y;
                self.waypoint_y = -temp_x;
            }
            _ => panic!("Invalid turn instruction"),
        }
    }

    fn travel(&mut self, multiplier: i32) {
        self.x += self.waypoint_x * multiplier;
        self.y += self.waypoint_y * multiplier;
    }

    fn move_waypoint(&mut self, cardinal: &Cardinal) {
        use Cardinal::*;

        match cardinal {
            North(dis) => self.waypoint_y += dis,
            East(dis) => self.waypoint_x += dis,
            South(dis) => self.waypoint_y -= dis,
            West(dis) => self.waypoint_x -= dis,
        }
    }

    pub fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

pub struct Instructions(Vec<Instruction>);

impl Instructions {
    pub fn from_reader<R: BufRead>(reader: R) -> Result<Self, AocError> {
        let instructions = reader
            .lines()
            .map(|line| match line {
                Ok(line) => line.trim().parse(),
                Err(e) => Err(AocError::from(e)),
            })
            .collect::<Result<_, _>>()?;

        Ok(Self(instructions))
    }
}

enum Instruction {
    Turn(Turn),
    Cardinal(Cardinal),
    Forward(i32),
}

impl FromStr for Instruction {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Cardinal::*;
        use Turn::*;
        let op = &s[0..1];
        let value = s[1..].parse().map_err(|_| AocError::InvalidInput)?;

        match op {
            "N" => Ok(Self::Cardinal(North(value))),
            "E" => Ok(Self::Cardinal(East(value))),
            "S" => Ok(Self::Cardinal(South(value))),
            "W" => Ok(Self::Cardinal(West(value))),
            "L" => Ok(Self::Turn(Left(value))),
            "R" => Ok(Self::Turn(Right(value))),
            "F" => Ok(Self::Forward(value)),
            _ => Err(AocError::InvalidInput),
        }
    }
}

enum Turn {
    Left(i32),
    Right(i32),
}

enum Cardinal {
    North(i32),
    East(i32),
    South(i32),
    West(i32),
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    fn get_input() -> impl BufRead {
        let input = "\
            F10
            N3
            F7
            R90
            F11\
        ";

        Cursor::new(input)
    }

    #[test]
    fn part_1() {
        let input = get_input();
        let instructions = Instructions::from_reader(input).unwrap();
        let mut boat = FirstBoat::new();
        boat.follow_instructions(&instructions);

        assert_eq!(boat.manhattan_distance(), 25);
    }

    #[test]
    fn part_2() {
        let input = get_input();
        let instructions = Instructions::from_reader(input).unwrap();
        let mut boat = SecondBoat::new();
        boat.follow_instructions(&instructions);

        assert_eq!(boat.manhattan_distance(), 286);
    }
}
