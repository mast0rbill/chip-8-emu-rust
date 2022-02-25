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
    
    //let mut file_buffer: Vec<u8> = Vec::new();

    let mut str_buffer = String::new();
    if let Ok(_) = file.read_to_string(&mut str_buffer) {
        println!("{}", str_buffer);
    }
}
