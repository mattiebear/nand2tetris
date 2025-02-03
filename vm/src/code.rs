use crate::parser::MathOp;

use super::parser::{Command, StackOp, StackSegment};

pub fn to_asm(command: &Command) -> Vec<String> {
    match command {
        Command::Stack {
            op,
            segment,
            value_or_index,
        } => match op {
            StackOp::Push => match segment {
                StackSegment::Constant => push_value(*value_or_index),
                _ => vec![],
            },
            _ => vec![],
        },
        Command::Arithmetic { op } => match op {
            MathOp::Add => vec![
                "// add".to_string(),
                "@SP".to_string(),
                "AM=M-1".to_string(),
                "D=M".to_string(),
                "A=A-1".to_string(),
                "M=D+M".to_string(),
            ],
            _ => vec![],
        },
        _ => vec![],
    }
}

pub fn end_loop() -> Vec<String> {
    vec![
        "// end loop".to_string(),
        "(END)".to_string(),
        "@END".to_string(),
        "0;JMP".to_string(),
    ]
}

fn push_value(value: u16) -> Vec<String> {
    vec![
        format!("// push constant {}", value),
        format!("@{}", value),
        "D=A".to_string(),
        "@SP".to_string(),
        "A=M".to_string(),
        "M=D".to_string(),
        "@SP".to_string(),
        "M=M+1".to_string(),
    ]
}
