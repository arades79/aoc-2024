use winnow::{
    ascii::{dec_uint, multispace1},
    combinator::{preceded, separated, separated_pair},
    prelude::*,
};

advent_of_code::solution!(17);

#[derive(Debug, Clone, Default)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    out: Vec<u64>,
    instructions: Box<[Operation]>,
    pc: usize,
}

impl Computer {
    fn new(a: u64, b: u64, c: u64, program: Box<[Operation]>) -> Self {
        Self {
            a,
            b,
            c,
            out: Vec::new(),
            instructions: program,
            pc: 0,
        }
    }
    fn process(&mut self, op: Operation) {
        let mut increment_pc = true;
        match op {
            Operation::Adv(combo) => {
                let value = self.combo(combo);
                let num = self.a;
                let denom = 1 << value;
                self.a = num / denom;
            }
            Operation::Bxl(Literal(literal)) => {
                self.b ^= literal;
            }
            Operation::Bst(combo) => {
                let value = self.combo(combo) % 8;
                self.b = value;
            }
            Operation::Jnz(Literal(literal)) => {
                if self.a != 0 {
                    self.pc = literal as usize >> 1;
                    increment_pc = false;
                }
            }
            Operation::Bxc(_) => {
                self.b ^= self.c;
            }
            Operation::Out(combo) => {
                let value = self.combo(combo) % 8;
                self.out.push(value);
            }
            Operation::Bdv(combo) => {
                let value = self.combo(combo);
                let num = self.a;
                let denom = 2u64.pow(value as u32);
                self.b = num / denom;
            }
            Operation::Cdv(combo) => {
                let value = self.combo(combo);
                let num = self.a;
                let denom = 2u64.pow(value as u32);
                self.c = num / denom;
            }
        };
        if increment_pc {
            self.pc += 1;
        }
        #[cfg(test)]
        println!(
            "a:{} b:{} c:{} pc:{} out:{:?} ({:?})",
            self.a, self.b, self.c, self.pc, &self.out, op
        )
    }
    fn execute(mut self) -> String {
        while self.pc < self.instructions.len() {
            self.process(self.instructions[self.pc]);
        }
        let mut s = String::new();
        for o in self.out {
            s.push(char::from_digit(o as u32, 10).unwrap());
            s.push(',')
        }
        s.pop();
        s
    }
    fn reg(&self, reg: Register) -> u64 {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
        }
    }
    fn combo(&self, operand: Combo) -> u64 {
        match operand {
            Combo::Immediate(Literal(literal)) => literal,
            Combo::Register(register) => self.reg(register),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Literal(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Register {
    A,
    B,
    C,
}

impl From<u64> for Register {
    fn from(value: u64) -> Self {
        match value {
            4 => Register::A,
            5 => Register::B,
            6 => Register::C,
            _ => panic!("register does not exist"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Combo {
    Immediate(Literal),
    Register(Register),
}

impl From<u64> for Combo {
    fn from(value: u64) -> Self {
        match value {
            i @ 0..4 => Combo::Immediate(Literal(i)),
            r @ 4..8 => Combo::Register(Register::from(r)),
            op => panic!("illegal operand {op}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Operation {
    Adv(Combo),
    Bxl(Literal),
    Bst(Combo),
    Jnz(Literal),
    Bxc(Literal),
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

impl From<(u64, u64)> for Operation {
    fn from(value: (u64, u64)) -> Self {
        use Operation::*;
        match value {
            (0, c @ 0..8) => Adv(Combo::from(c)),
            (1, l @ 0..8) => Bxl(Literal(l)),
            (2, c @ 0..8) => Bst(Combo::from(c)),
            (3, l @ 0..8) => Jnz(Literal(l)),
            (4, l @ 0..8) => Bxc(Literal(l)),
            (5, c @ 0..8) => Out(Combo::from(c)),
            (6, c @ 0..8) => Bdv(Combo::from(c)),
            (7, c @ 0..8) => Cdv(Combo::from(c)),
            (op, sub) => panic!("Illegal operation {op} or operand {sub}"),
        }
    }
}

fn operation_parser(input: &mut &str) -> PResult<Operation> {
    let op = separated_pair(dec_uint, ",", dec_uint).parse_next(input)?;
    Ok(Operation::from(op))
}

fn program_parser(input: &mut &str) -> PResult<Vec<Operation>> {
    preceded("Program: ", separated(1.., operation_parser, ",")).parse_next(input)
}

fn computer_parser(input: &mut &str) -> PResult<Computer> {
    let (a, b, c) = (
        preceded("Register A: ", dec_uint),
        preceded("\nRegister B: ", dec_uint),
        preceded("\nRegister C: ", dec_uint),
    )
        .parse_next(input)?;
    let program = preceded(multispace1, program_parser)
        .parse_next(input)?
        .into_boxed_slice();
    Ok(Computer::new(a, b, c, program))
}

fn hardcoded() {
    let mut a = 47792830;
    let mut b = 0;
    let mut c = 0;
    let mut out = Vec::new();
    while a > 0 {
        b = a % 8;
        b ^= 5;
        c = a / (1 << b);
        b ^= 6;
        b ^= c;
        out.push(b % 8);
        a /= 8;
    }
    println!("hardcoded: {out:?}");
}

pub fn part_one(input: &str) -> Option<String> {
    let computer = computer_parser.parse(input).ok()?;
    let out = computer.execute();
    hardcoded();
    Some(out)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
