extern crate sdl2;

use crate::chip8;
use sdl2::pixels::Color;

pub struct Chip8Display {
    square_size: u32,
    canvas: sdl2::render::WindowCanvas,
}

impl Chip8Display {
    pub fn new(context: &sdl2::Sdl, title: &str, square_size: u32) -> Chip8Display {
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem.window(
            title, 
            chip8::Chip8::VIDEO_WIDTH as u32 * square_size, 
            chip8::Chip8::VIDEO_HEIGHT as u32 * square_size
        )
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        return Chip8Display {
            square_size,
            canvas,
        };
    }

    pub fn draw(&mut self, chip8: &chip8::Chip8) {
        // Clear canvas
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        // Draw pixels
        for y in 0..chip8::Chip8::VIDEO_HEIGHT {
            for x in 0..chip8::Chip8::VIDEO_WIDTH {
                let pixel = chip8.get_video(x, y);
                if pixel == 0 {
                    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                } else {
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                }
                self.canvas.fill_rect(
                    sdl2::rect::Rect::new(
                        x as i32 * self.square_size as i32,
                        y as i32 * self.square_size as i32,
                        self.square_size,
                        self.square_size,
                    )
                ).unwrap();
            }
        }

        self.canvas.present();
    }
}

