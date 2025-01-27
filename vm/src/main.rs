mod parser;

use parser::Parser;
use std::fs::File;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let file = File::open(&args[1]).unwrap();
    let mut parser = Parser::new(&file);

    while parser.has_more_lines() {
        let current = parser.current();

        println!("{:?}", current);

        parser.advance();
    }
}
