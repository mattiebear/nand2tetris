use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Parser {
    lines: Vec<String>,
    read_line: i32,
}

impl Parser {
    pub fn new(file: &File) -> Self {
        let lines = BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .filter(|l| !l.starts_with("//") && l.len() > 0)
            .collect();

        Self {
            lines,
            read_line: -1,
        }
    }

    pub fn current_line(&self) -> String {
        self.lines[self.read_line as usize].clone()
    }

    pub fn has_more_lines(&self) -> bool {
        let total_lines = self.lines.len() - 1;
        self.read_line < total_lines.try_into().unwrap()
    }

    pub fn advance(&mut self) {
        self.read_line += 1;
    }

    pub fn instruction_type(&self) -> InstructionType {
        let current_line = self.current_line();

        if current_line.starts_with("@") {
            return InstructionType::A;
        } else if current_line.starts_with("(") {
            return InstructionType::L;
        } else {
            return InstructionType::C;
        }
    }

    pub fn symbol(&self) -> String {
        if self.current_line().starts_with("@") {
            return self.current_line().replace("@", "");
        } else {
            return self.current_line().replace("(", "").replace(")", "");
        }
    }

    pub fn dest(&self) -> String {
        let line = self.current_line();
        let targ = line.split("=").next().unwrap();

        targ.to_string()
    }

    pub fn comp(&self) -> String {
        let line = self.current_line();
        let targ = line.split("=").last().unwrap();
        let targ = targ.split(";").next().unwrap();

        targ.to_string()
    }

    pub fn jump(&self) -> String {
        let line = self.current_line();

        if !line.contains(";") {
            return "".to_string();
        }

        let targ = line.split("=").last().unwrap();
        let targ = targ.split(";").last().unwrap();

        targ.to_string()
    }
}

#[derive(Debug)]
pub enum InstructionType {
    A,
    C,
    L,
}
