use std::convert::{TryFrom, TryInto};
use std::default::Default;
use std::io::BufRead;
use std::iter;
use thiserror::Error;

/// Order of directions is important for line of sight in part_2
static DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Error, Debug)]
pub enum AocError {
    #[error("Unrecognized input error")]
    ParseInputError,
    #[error("Error reading input")]
    IoError(#[from] std::io::Error),
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Space {
    Empty,
    Floor,
    Taken,
}

impl TryFrom<u8> for Space {
    type Error = AocError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'L' => Ok(Self::Empty),
            b'.' => Ok(Self::Floor),
            b'#' => Ok(Self::Taken),
            _ => Err(AocError::ParseInputError),
        }
    }
}

impl Default for Space {
    fn default() -> Self {
        Self::Floor
    }
}

/// positions includes a layer of padding (Floor) around the seating area. Width and height also
/// include the padding
#[derive(Clone)]
pub struct Seating {
    positions: Vec<Vec<Space>>,
}

impl Seating {
    pub fn from_reader<R: BufRead>(reader: R) -> Result<Self, AocError> {
        let mut positions: Vec<Vec<Space>> = reader
            .lines()
            .map(|line| match line {
                Ok(line) => iter::once(b'.')
                    .chain(line.trim().bytes())
                    .chain(iter::once(b'.'))
                    .map(|b| b.try_into())
                    .collect::<Result<_, _>>(),
                Err(e) => Err(AocError::from(e)),
            })
            .collect::<Result<_, _>>()?;

        let width = positions[0].len();
        let padding = vec![Space::Floor; width];
        positions.push(padding.clone());
        positions.push(padding);
        positions.rotate_right(1);

        Ok(Self { positions })
    }

    /// Updates the seating according to how many taken seats are around each position, if the
    /// seating changes then it returns true, if it's the same then it returns false
    fn step(&mut self, counts: &Vec<Vec<u32>>, taken: u32) -> bool {
        let mut changed = false;

        counts.iter().enumerate().for_each(|(j, row)| {
            row.iter().enumerate().for_each(|(i, &count)| {
                let space = self
                    .positions
                    .get_mut(j + 1)
                    .unwrap()
                    .get_mut(i + 1)
                    .unwrap();

                match space {
                    Space::Empty if count == 0 => {
                        *space = Space::Taken;
                        changed = true;
                    }
                    Space::Taken if count >= taken => {
                        *space = Space::Empty;
                        changed = true;
                    }
                    _ => {}
                }
            });
        });

        changed
    }

    pub fn process_adjacent(&mut self) {
        loop {
            let counts = self.build_counts_adjacent();
            if !self.step(&counts, 4) {
                break;
            }
        }
    }

    pub fn process_los(&mut self) {
        loop {
            let counts = self.build_counts_los();
            if !self.step(&counts, 5) {
                break;
            }
        }
    }

    pub fn num_seats(&self) -> u32 {
        self.positions.iter().flatten().fold(0, |acc, seat| {
            if *seat == Space::Taken {
                return acc + 1;
            }
            acc
        })
    }

    /// builds a structure that counts all of the adjacent taken seats for a given position
    fn build_counts_adjacent(&self) -> Vec<Vec<u32>> {
        let counts: Vec<Vec<u32>> = self.positions[1..self.positions.len() - 1]
            .iter()
            .enumerate()
            .map(|(j, row)| {
                row[1..row.len() - 1]
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        DIRECTIONS.iter().fold(0, |acc, direction| {
                            // + 1 to x and y as we are starting iteration from offset of 1
                            let x = (i as i32 + direction.0 + 1) as usize;
                            let y = (j as i32 + direction.1 + 1) as usize;

                            if self.positions[y][x] == Space::Taken {
                                return acc + 1;
                            }
                            acc
                        })
                    })
                    .collect()
            })
            .collect();

        counts
    }

    /// builds a structure that counts the taken seats that are in line of sight.
    fn build_counts_los(&self) -> Vec<Vec<u32>> {
        // Include the buffer from positions
        let mut los =
            vec![vec![FloorLos::default(); self.positions[0].len()]; self.positions.len()];

        // iterate forwards through the positions looking behind to build up lines of sight for w,
        // nw, n, and ne positions
        for (j, row) in self.positions[1..self.positions.len() - 1]
            .iter()
            .enumerate()
        {
            for (i, &space) in row[1..row.len() - 1].iter().enumerate() {
                if space == Space::Floor {
                    for direction in DIRECTIONS[0..4].iter() {
                        let x = (i as i32 + direction.0 + 1) as usize;
                        let y = (j as i32 + direction.1 + 1) as usize;

                        match self.positions[y][x] {
                            Space::Empty => {
                                *los[j + 1][i + 1].get_mut(direction) = Space::Empty;
                            }
                            Space::Floor => {
                                *los[j + 1][i + 1].get_mut(direction) = *los[y][x].get(direction);
                            }
                            Space::Taken => {
                                *los[j + 1][i + 1].get_mut(direction) = Space::Taken;
                            }
                        }
                    }
                }
            }
        }

        // iterate backwards through the positions looking behind to build up lines of sight for e,
        // se, s, and sw positions
        for (j, row) in self.positions[1..self.positions.len() - 1]
            .iter()
            .enumerate()
            .rev()
        {
            for (i, &space) in row[1..row.len() - 1].iter().enumerate().rev() {
                if space == Space::Floor {
                    for direction in DIRECTIONS[4..].iter() {
                        let x = (i as i32 + direction.0 + 1) as usize;
                        let y = (j as i32 + direction.1 + 1) as usize;

                        match self.positions[y][x] {
                            Space::Empty => {
                                *los[j + 1][i + 1].get_mut(direction) = Space::Empty;
                            }
                            Space::Floor => {
                                *los[j + 1][i + 1].get_mut(direction) = *los[y][x].get(direction);
                            }
                            Space::Taken => {
                                *los[j + 1][i + 1].get_mut(direction) = Space::Taken;
                            }
                        }
                    }
                }
            }
        }

        // now what we have all of the lines of sight we can count the taken chairs around the
        // given position
        let counts: Vec<Vec<u32>> = self.positions[1..self.positions.len() - 1]
            .iter()
            .enumerate()
            .map(|(j, row)| {
                row[1..row.len() - 1]
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        DIRECTIONS.iter().fold(0, |acc, direction| {
                            // + 1 to x and y as we are starting iteration from offset of 1
                            let x = (i as i32 + direction.0 + 1) as usize;
                            let y = (j as i32 + direction.1 + 1) as usize;

                            match self.positions[y][x] {
                                Space::Taken => acc + 1,
                                Space::Floor if *los[y][x].get(direction) == Space::Taken => {
                                    acc + 1
                                }
                                _ => acc,
                            }
                        })
                    })
                    .collect()
            })
            .collect();

        counts
    }
}

/// Wasted 9th element in the middle to make fetching the correct item easier
#[derive(Default, Clone, Copy)]
struct FloorLos([Space; 9]);

impl FloorLos {
    fn get(&self, direction: &(i32, i32)) -> &Space {
        let index = (direction.0 + 1 + (direction.1 + 1) * 3) as usize;

        &self.0[index]
    }

    fn get_mut(&mut self, direction: &(i32, i32)) -> &mut Space {
        let index = (direction.0 + 1 + (direction.1 + 1) * 3) as usize;

        &mut self.0[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn part_1() {
        let input = "\
            L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL\
        ";

        let input = Cursor::new(input);
        let mut seating = Seating::from_reader(input).unwrap();

        seating.process_adjacent();
        let seats = seating.num_seats();

        assert_eq!(seats, 37);
    }

    #[test]
    fn part_2() {
        let input = "\
            L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL\
        ";

        let input = Cursor::new(input);
        let mut seating = Seating::from_reader(input).unwrap();

        seating.process_los();
        let seats = seating.num_seats();

        assert_eq!(seats, 26);
    }
}
