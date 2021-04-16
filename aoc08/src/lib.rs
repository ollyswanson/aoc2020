use once_cell::sync::OnceCell;
use regex::Regex;
use std::collections::HashSet;
use std::io::BufRead;
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(PartialEq)]
pub struct Executable(Vec<Op>);

impl Executable {
    pub fn from_reader<R: BufRead>(reader: &mut R) -> Result<Self> {
        let mut buffer = String::new();
        let mut ops = Vec::new();

        loop {
            match reader.read_line(&mut buffer) {
                Ok(i) if i == 0 => {
                    break;
                }
                Ok(_) => {
                    ops.push(buffer.parse()?);
                    buffer.clear();
                }
                Err(_) => {
                    break;
                }
            }
        }

        Ok(Self(ops))
    }

    fn swap_op(&mut self, line: usize) -> Result<()> {
        let op = self.0.get_mut(line).ok_or("Invalid line number")?;

        match *op {
            Op::Jmp(i) => *op = Op::Nop(i),
            Op::Nop(i) => *op = Op::Jmp(i),
            _ => {}
        }

        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for Op {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        static RE: OnceCell<Regex> = OnceCell::new();
        let regex =
            RE.get_or_init(|| Regex::new(r"(?P<instr>[a-z]{3})\s(?P<val>(\+|-)\d+)").unwrap());

        let caps = regex.captures(s).ok_or("invalid expression")?;

        let val = caps["val"].parse()?;

        match &caps["instr"] {
            "acc" => Ok(Self::Acc(val)),
            "jmp" => Ok(Self::Jmp(val)),
            "nop" => Ok(Self::Nop(val)),
            _ => Err("unrecognized operation".into()),
        }
    }
}

#[derive(PartialEq)]
pub enum Termination {
    Loop,
    Eof,
}

pub struct Program<'a> {
    exe: &'a mut Executable,
    pub acc: i32,
    pc: usize,
}

impl<'a> Program<'a> {
    pub fn new(exe: &'a mut Executable) -> Self {
        Self { exe, acc: 0, pc: 0 }
    }

    pub fn reset(&mut self) {
        self.acc = 0;
        self.pc = 0;
    }

    fn op(&mut self) {
        match self.exe.0[self.pc] {
            Op::Acc(i) => {
                self.pc += 1;
                self.acc += i;
            }
            Op::Jmp(i) => {
                // Fix this
                self.pc = (self.pc as i32 + i) as usize;
            }
            Op::Nop(_) => {
                self.pc += 1;
            }
        };
    }

    /// Executes until an instruction is repeated and then stops
    pub fn execute_from(&mut self, start: usize) -> Termination {
        self.pc = start;
        let mut executed = HashSet::new();
        let eof = self.exe.0.len();

        loop {
            if self.pc >= eof {
                return Termination::Eof;
            }
            if !executed.insert(self.pc) {
                return Termination::Loop;
            }
            self.op()
        }
    }

    pub fn repair_executable(&mut self) -> Result<()> {
        for i in 0..self.exe.0.len() {
            match self.exe.0[i] {
                Op::Jmp(_) | Op::Nop(_) => {
                    // save program counter and executable so that we don't have to resume from the
                    // beginning
                    let acc = self.acc;
                    let pc = self.pc;

                    // Swap and try
                    self.exe.swap_op(i)?;

                    if self.execute_from(pc) == Termination::Loop {
                        // swap back and restore
                        self.acc = acc;
                        self.pc = pc;
                        self.exe.swap_op(i)?;
                    } else {
                        return Ok(());
                    }
                }
                _ => {}
            }
        }

        Err("Unfixable".into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parses_executable() {
        use Op::*;

        let ops = "\
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6";
        let mut ops = Cursor::new(ops);

        let exe = vec![
            Nop(0),
            Acc(1),
            Jmp(4),
            Acc(3),
            Jmp(-3),
            Acc(-99),
            Acc(1),
            Jmp(-4),
            Acc(6),
        ];

        assert_eq!(exe, Executable::from_reader(&mut ops).unwrap().0);
    }

    #[test]
    fn part_1() {
        let ops = "\
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6";
        let mut ops = Cursor::new(ops);

        let mut exe = Executable::from_reader(&mut ops).unwrap();
        let mut program = Program::new(&mut exe);

        program.execute_from(0);

        assert_eq!(5, program.acc);
    }

    #[test]
    fn part_2() {
        let ops = "\
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6";
        let mut ops = Cursor::new(ops);

        let mut exe = Executable::from_reader(&mut ops).unwrap();
        let mut program = Program::new(&mut exe);

        program.repair_executable().unwrap();
        assert_eq!(8, program.acc);
    }
}
