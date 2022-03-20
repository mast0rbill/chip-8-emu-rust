extern crate sdl2;

use std::io::{Read, Write};

use std::io::Stdout;

use std::fs::File;
use std::env;

mod chip8;
mod display;

use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


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
    file.read_to_end(&mut buffer).unwrap();

    let mut chip = chip8::Chip8::new(&buffer);
    let sdl_context = sdl2::init().unwrap();
    let mut display = display::Chip8Display::new(&sdl_context, "Chip8", 24); 
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        // Input handling
        /*
        Keypad       Keyboard
        +-+-+-+-+    +-+-+-+-+
        |1|2|3|C|    |1|2|3|4|
        +-+-+-+-+    +-+-+-+-+
        |4|5|6|D|    |Q|W|E|R|
        +-+-+-+-+ => +-+-+-+-+
        |7|8|9|E|    |A|S|D|F|
        +-+-+-+-+    +-+-+-+-+
        |A|0|B|F|    |Z|X|C|V|
        +-+-+-+-+    +-+-+-+-+
        */
        for event in event_pump.poll_iter() {
            // Input handling
            match event {
                Event::Quit {..} => break 'running,
                Event::KeyDown { keycode, .. } =>
                    match keycode {
                        Some(Keycode::Escape) => break 'running,
                        Some(Keycode::X) => chip.set_key(0x0, true),
                        Some(Keycode::Num1) => chip.set_key(0x1, true),
                        Some(Keycode::Num2) => chip.set_key(0x2, true),
                        Some(Keycode::Num3) => chip.set_key(0x3, true),
                        Some(Keycode::Q) => chip.set_key(0x4, true),
                        Some(Keycode::W) => chip.set_key(0x5, true),
                        Some(Keycode::E) => chip.set_key(0x6, true),
                        Some(Keycode::A) => chip.set_key(0x7, true),
                        Some(Keycode::S) => chip.set_key(0x8, true),
                        Some(Keycode::D) => chip.set_key(0x9, true),
                        Some(Keycode::Z) => chip.set_key(0xA, true),
                        Some(Keycode::C) => chip.set_key(0xB, true),
                        Some(Keycode::Num4) => chip.set_key(0xC, true),
                        Some(Keycode::R) => chip.set_key(0xD, true),
                        Some(Keycode::F) => chip.set_key(0xE, true),
                        Some(Keycode::V) => chip.set_key(0xF, true),
                        _ => {},
                    }

                Event::KeyUp { keycode, .. } =>
                    match keycode {
                        Some(Keycode::X) => chip.set_key(0x0, false),
                        Some(Keycode::Num1) => chip.set_key(0x1, false),
                        Some(Keycode::Num2) => chip.set_key(0x2, false),
                        Some(Keycode::Num3) => chip.set_key(0x3, false),
                        Some(Keycode::Q) => chip.set_key(0x4, false),
                        Some(Keycode::W) => chip.set_key(0x5, false),
                        Some(Keycode::E) => chip.set_key(0x6, false),
                        Some(Keycode::A) => chip.set_key(0x7, false),
                        Some(Keycode::S) => chip.set_key(0x8, false),
                        Some(Keycode::D) => chip.set_key(0x9, false),
                        Some(Keycode::Z) => chip.set_key(0xA, false),
                        Some(Keycode::C) => chip.set_key(0xB, false),
                        Some(Keycode::Num4) => chip.set_key(0xC, false),
                        Some(Keycode::R) => chip.set_key(0xD, false),
                        Some(Keycode::F) => chip.set_key(0xE, false),
                        Some(Keycode::V) => chip.set_key(0xF, false),
                        _ => {},
                    }
                _ => {}
            }
        }

        chip.cycle();
        
        display.draw(&mut chip);
        std::thread::sleep(Duration::new(0, 1_000_000_000 / 120));
    }
}
