use std::io::{Read};
use std::fs::File;
use std::env;

mod chip8;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: [file_name]. Got {:?}", args);
    }

    let file_name = &args[1];
    let mut file = match File::open(file_name) {
        Ok(f) => f,
        Err(e) => panic!("Error opening file {}: {}", file_name, e),
    };

    let mut buffer: Vec<u8> = Vec::new();
    if let Err(e) = file.read(&mut buffer) {
        panic!("{}", e)
    }

    let chip = chip8::Chip8::new(&buffer);
}
