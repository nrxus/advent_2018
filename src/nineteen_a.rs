#![feature(try_trait)]

use std::{num::ParseIntError, option::NoneError, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

fn solve(input: &str) -> u32 {
    Program::from_str(input).unwrap().run()
}

#[derive(Debug)]
struct Program {
    lines: Vec<Line>,
    registers: Registers,
}

impl Program {
    fn run(&mut self) -> u32 {
        while self.registers.instruction() < self.lines.len() {
            let line = &self.lines[self.registers.instruction()];
            self.registers.calculate(line);
            self.registers.increase_instruction();
        }
        self.registers.regs[0]
    }
}

#[derive(Debug)]
struct Line {
    instruction: Instruction,
    a: u32,
    b: u32,
    c: u32,
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Registers {
    ip: usize,
    regs: [u32; 6],
}

impl Registers {
    fn calculate(&mut self, line: &Line) {
        let &Line {
            instruction,
            a,
            b,
            c,
        } = line;

        self.regs[c as usize] = match instruction {
            Instruction::AddR => self.regs[a as usize] + self.regs[b as usize],
            Instruction::AddI => self.regs[a as usize] + b,
            Instruction::MulR => self.regs[a as usize] * self.regs[b as usize],
            Instruction::MulI => self.regs[a as usize] * b,
            Instruction::BandR => self.regs[a as usize] & self.regs[b as usize],
            Instruction::BandI => self.regs[a as usize] & b,
            Instruction::BorR => self.regs[a as usize] | self.regs[b as usize],
            Instruction::BorI => self.regs[a as usize] | b,
            Instruction::SetR => self.regs[a as usize],
            Instruction::SetI => a,
            Instruction::GtIR => (a > self.regs[b as usize]) as u32,
            Instruction::GtRI => (self.regs[a as usize] > b) as u32,
            Instruction::GtRR => (self.regs[a as usize] > self.regs[b as usize]) as u32,
            Instruction::EqIR => (a == self.regs[b as usize]) as u32,
            Instruction::EqRI => (self.regs[a as usize] == b) as u32,
            Instruction::EqRR => (self.regs[a as usize] == self.regs[b as usize]) as u32,
        };
    }

    fn instruction(&self) -> usize {
        self.regs[self.ip] as usize
    }

    fn increase_instruction(&mut self) {
        self.regs[self.ip] += 1;
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    AddR,
    AddI,
    MulR,
    MulI,
    BandR,
    BandI,
    BorR,
    BorI,
    SetR,
    SetI,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
}

impl FromStr for Program {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut input = input.lines();
        let ip: usize = Regex::new(r"#ip (?P<ip>\d+)")?
            .captures(input.next()?)?
            .name("ip")?
            .as_str()
            .parse()?;
        let lines = input.map(|l| Line::from_str(l)).collect::<Result<_, _>>()?;
        let registers = Registers {
            ip,
            regs: [0, 0, 0, 0, 0, 0],
        };

        Ok(Program { lines, registers })
    }
}

impl FromStr for Line {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?P<instruction>.*) (?P<a>\d+) (?P<b>\d+) (?P<c>\d+)").unwrap();
        }

        let caps = RE.captures(input)?;
        let instruction = caps.name("instruction")?.as_str();
        let a: u32 = caps.name("a")?.as_str().parse()?;
        let b: u32 = caps.name("b")?.as_str().parse()?;
        let c: u32 = caps.name("c")?.as_str().parse()?;

        let instruction = match instruction {
            "addr" => Instruction::AddR,
            "addi" => Instruction::AddI,
            "mulr" => Instruction::MulR,
            "muli" => Instruction::MulI,
            "banr" => Instruction::BandR,
            "bani" => Instruction::BandI,
            "borr" => Instruction::BorR,
            "bori" => Instruction::BorI,
            "setr" => Instruction::SetR,
            "seti" => Instruction::SetI,
            "gtir" => Instruction::GtIR,
            "gtri" => Instruction::GtRI,
            "gtrr" => Instruction::GtRR,
            "eqir" => Instruction::EqIR,
            "eqri" => Instruction::EqRI,
            "eqrr" => Instruction::EqRR,
            _ => Err(ParsingError)?,
        };

        Ok(Line {
            instruction,
            a,
            b,
            c,
        })
    }
}

#[derive(Debug)]
struct ParsingError;

impl From<regex::Error> for ParsingError {
    fn from(_: regex::Error) -> Self {
        ParsingError
    }
}

impl From<NoneError> for ParsingError {
    fn from(_: NoneError) -> Self {
        ParsingError
    }
}

impl From<ParseIntError> for ParsingError {
    fn from(_: ParseIntError) -> Self {
        ParsingError
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";
        assert_eq!(solve(input), 7);
    }
}

common::read_main!();
//common::bootstrap!(16);
