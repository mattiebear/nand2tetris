use std::fs::File;
use std::io::{BufRead, BufReader};

pub enum MathOp {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

#[derive(Debug)]
pub enum StackOp {
    Push,
    Pop,
}

#[derive(Debug)]
pub enum StackSegment {
    Constant,
    Local,
    Argument,
    This,
    That,
    Temp,
}

pub enum Command {
    Arithmetic {
        op: MathOp,
    },
    Stack {
        op: StackOp,
        segment: StackSegment,
        value_or_index: u16,
    },
}

pub struct Parser {
    lines: Vec<String>,
    pos: u16,
}

impl Parser {
    pub fn new(file: &File) -> Self {
        let lines = BufReader::new(file)
            .lines()
            .map(|l| l.unwrap().trim().to_string())
            .filter(|l| !l.starts_with("//") && l.len() > 0)
            .collect();

        Self { lines, pos: 0 }
    }

    pub fn has_more_lines(self: &Self) -> bool {
        self.pos < self.total_lines()
    }

    pub fn advance(self: &mut Self) {
        self.pos += 1;
    }

    pub fn current(self: &Self) -> Command {
        let string = self.lines[self.pos as usize].clone();
        let segments: Vec<&str> = string.split_whitespace().collect();

        if segments.len() == 1 {
            let op = match segments[0] {
                "add" => MathOp::Add,
                "sub" => MathOp::Sub,
                "neg" => MathOp::Neg,
                "eq" => MathOp::Eq,
                "gt" => MathOp::Gt,
                "lt" => MathOp::Lt,
                "and" => MathOp::And,
                "or" => MathOp::Or,
                "not" => MathOp::Not,
                _ => panic!("Invalid arithmetic operation"),
            };

            Command::Arithmetic { op }
        } else {
            let op = match segments[0] {
                "push" => StackOp::Push,
                "pop" => StackOp::Pop,
                _ => panic!("Invalid stack operation"),
            };

            let segment = match segments[1] {
                "constant" => StackSegment::Constant,
                "local" => StackSegment::Local,
                "argument" => StackSegment::Argument,
                "this" => StackSegment::This,
                "that" => StackSegment::That,
                "temp" => StackSegment::Temp,
                _ => panic!("Invalid stack segment"),
            };

            let value_or_index = segments[2].parse::<u16>().unwrap();

            Command::Stack {
                op,
                segment,
                value_or_index,
            }
        }
    }

    fn total_lines(self: &Self) -> u16 {
        self.lines.len() as u16
    }
}
