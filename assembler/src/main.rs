mod code;
mod parser;
mod symbol_lookup;

use parser::{InstructionType, Parser};
use std::fs::File;
use std::io::Write;
use symbol_lookup::SymbolLookup;

fn main() {
    // Get the argv from the command line
    let args: Vec<String> = std::env::args().collect();

    // Check if the user has provided a file name
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let file = File::open(&args[1]).unwrap();
    let mut parser = Parser::new(&file);

    let mut symbol_lookup = SymbolLookup::new();

    // First pass, get all of the labels
    while parser.has_more_lines() {
        parser.advance();

        match parser.instruction_type() {
            InstructionType::L => {
                symbol_lookup.set_line(parser.symbol(), parser.register_number as u16 + 1);
            }
            _ => (),
        }
    }

    parser.reset();

    let mut instructions: Vec<String> = vec![];

    while parser.has_more_lines() {
        parser.advance();

        match parser.instruction_type() {
            InstructionType::C => {
                let instruction = String::from("111");
                let dest = code::dest(&parser.dest());
                let comp = code::comp(&parser.comp());
                let jump = code::jump(&parser.jump());

                instructions.push(instruction + &comp + &dest + &jump);
            }
            InstructionType::A => {
                let symbol = parser.symbol();

                if symbol.chars().all(char::is_numeric) {
                    let bit_symbol = format!("{:016b}", symbol.parse::<u16>().unwrap());
                    instructions.push(bit_symbol);
                    continue;
                } else {
                    if !symbol_lookup.contains(&symbol) {
                        symbol_lookup.add_variable(symbol.clone());
                    }

                    let address = symbol_lookup.get(&symbol).unwrap();
                    let bit_address = format!("{:016b}", address);
                    instructions.push(bit_address);
                }
            }
            InstructionType::L => (),
        }
    }

    let bin_file_name = &args[1].replace("asm", "hack");
    let mut bin_file = File::create(bin_file_name).unwrap();

    for instruction in instructions {
        bin_file.write_all(instruction.as_bytes()).unwrap();
        bin_file.write_all(b"\n").unwrap();
    }
}
