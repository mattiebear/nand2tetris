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
                StackSegment::Constant => vec![
                    format!("// push constant {}", value_or_index),
                    format!("@{}", value_or_index),
                    "D=A".to_string(),
                    "@SP".to_string(),
                    "A=M".to_string(),
                    "M=D".to_string(),
                    "@SP".to_string(),
                    "M=M+1".to_string(),
                ],
                StackSegment::Temp => vec![
                    format!("// push temp {}", value_or_index),
                    format!("@{}", 5 + value_or_index),
                    "D=M".to_string(),
                    "@SP".to_string(),
                    "A=M".to_string(),
                    "M=D".to_string(),
                    "@SP".to_string(),
                    "M=M+1".to_string(),
                ],
                // All other segments need to be handled by first finding the
                // address of the segment and then adding the index to it.
                _ => {
                    let segment = match segment {
                        StackSegment::Local => "LCL",
                        StackSegment::Argument => "ARG",
                        StackSegment::This => "THIS",
                        StackSegment::That => "THAT",
                        _ => panic!("Invalid segment"),
                    };

                    vec![
                        format!("// push {} {}", segment, value_or_index),
                        format!("@{}", value_or_index),
                        "D=A".to_string(),
                        format!("@{}", segment),
                        "A=M+D".to_string(),
                        "D=M".to_string(),
                        "@SP".to_string(),
                        "A=M".to_string(),
                        "M=D".to_string(),
                        "@SP".to_string(),
                        "M=M+1".to_string(),
                    ]
                }
            },
            StackOp::Pop => match segment {
                StackSegment::Temp => vec![
                    format!("// pop temp {}", value_or_index),
                    "@SP".to_string(),
                    "AM=M-1".to_string(),
                    "D=M".to_string(),
                    format!("@{}", 5 + value_or_index),
                    "M=D".to_string(),
                ],
                _ => {
                    let segment = match segment {
                        StackSegment::Local => "LCL",
                        StackSegment::Argument => "ARG",
                        StackSegment::This => "THIS",
                        StackSegment::That => "THAT",
                        _ => panic!("Invalid segment"),
                    };

                    vec![
                        format!("// pop {} {}", segment, value_or_index),
                        format!("@{}", value_or_index),
                        "D=A".to_string(),
                        format!("@{}", segment),
                        "D=M+D".to_string(),
                        "@R13".to_string(),
                        "M=D".to_string(),
                        "@SP".to_string(),
                        "AM=M-1".to_string(),
                        "D=M".to_string(),
                        "@R13".to_string(),
                        "A=M".to_string(),
                        "M=D".to_string(),
                    ]
                }
            },
            _ => panic!("Invalid stack operation"),
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
            MathOp::Sub => vec![
                "// sub".to_string(),
                "@SP".to_string(),
                "AM=M-1".to_string(),
                "D=M".to_string(),
                "A=A-1".to_string(),
                "M=M-D".to_string(),
            ],
            MathOp::Neg => vec![
                "// neg".to_string(),
                "@SP".to_string(),
                "A=M-1".to_string(),
                "M=-M".to_string(),
            ],
            _ => panic!("Invalid arithmetic operation"),
        },
        _ => panic!("Invalid command"),
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

fn push_value(value: u16) -> Vec<String> {}
