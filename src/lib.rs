use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use std::time::Duration;
use sdl2::rect;

pub mod chip8;
use crate::chip8::{Chip8, Chip8Display};


/// open sdl2 window and create a new chip8 display
/// draw display to screen in a loop
/// display might onl be updated when necessary instead of 60 FPS for better optimization
pub fn open_window() -> Result<(), std::io::Error> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Chip-8 emulator", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut my_chip8 = Chip8::new_default();
    my_chip8.draw_sprite_in_mem_to_x_y(0x50, 0, 0, 5, &mut canvas);
    my_chip8.draw_sprite_in_mem_to_x_y(0x55, 9, 0, 5, &mut canvas);
    match my_chip8.load_file_to_mem() {
        Err(e) => return Err(e),
        _ => {}
    };
    my_chip8.get_mem_state();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.clear();
                    canvas.present();
                },
                Event::KeyDown { keycode: Some(Keycode::B), .. } => {
                    draw_display_to_window(&mut canvas, &my_chip8.display);
                }
                _ => {}
            }
        }

        //60 FPS        
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}


/// draws the pixels of Chip8isplay to SDL2 canvas
/// pixels are represented as 10x10 rectangles
/// called by the running loop of canvas window
fn draw_display_to_window(canvas: &mut WindowCanvas, disp: &Chip8Display) {
    let mut y = 0;
    canvas.set_draw_color(Color::RGB(100, 225, 0));
    for row in disp.pixels {
        let mut x = 0;
        for byte in row {
            let mut mask = 0b10000000;
            for _i in 0..8 {
                if byte & mask == mask {
                    let rect = rect::Rect::new(x, y, 10, 10);
                    canvas.draw_rect(rect).unwrap();
                    canvas.fill_rect(rect).unwrap();
                } 
                mask = mask  >> 1;
                x += 10;
            }
        }
        y += 10;
    }
    canvas.present();
}

