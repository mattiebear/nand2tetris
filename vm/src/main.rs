mod code;
mod parser;

use parser::Parser;
use std::{fs::File, io::Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let file = File::open(&args[1]).unwrap();
    let mut parser = Parser::new(&file);
    let mut asm: Vec<String> = Vec::new();

    while parser.has_more_lines() {
        let command = parser.current();
        let asm_lines = code::to_asm(&command);

        // Add asm_lines to asm
        asm.extend(asm_lines);
        parser.advance();
    }

    asm.extend(code::end_loop());

    // Write to .asm file
    let bin_file_name = &args[1].replace("vm", "asm");
    let mut bin_file = File::create(bin_file_name).unwrap();

    for line in asm {
        bin_file.write_all(line.as_bytes()).unwrap();
        bin_file.write_all(b"\n").unwrap();
    }
}
