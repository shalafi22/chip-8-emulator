use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use std::time::Duration;
use sdl2::rect;

pub struct Chip8 {
    //The main memory of the machine, 4KB
    pub memory: [u8; 4096],

    //16 General purpose 8-bit registers V1 - Vf,
    //Vf isn't used by programs, it is used by some instructions as flag
    pub Vx: [u8 ;16],

    //16-bit special register
    pub I: u16,

    //TODO: These registers are decremented at a rate of 60Hz when non-zero
    pub delay_timer: u8,
    pub sound_timer: u8,

    //Program Counter and Stack Pointer
    pub PC: u16,
    pub SP: u8,

    //Stack, allows max 16 subroutines
    pub stack: [u16; 16],
}

pub struct Chip8Display {
    pub pixels: [[u8; 8];32]
}

/// open sdl2 window and create a new chip8 display
/// draw display to screen in a loop
/// display might onl be updated when necessary instead of 60 FPS for better optimization
fn open_window(disp: &Chip8Display) {
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
                    draw_display_to_window(&mut canvas, &disp);
                }
                _ => {}
            }
        }

        
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


/// draws the pixels of Chip8isplay to SDL2 canvas
/// pixels are represented as 10x10 rectangles
/// called by the running loop of canvas window
fn draw_display_to_window(canvas: &mut WindowCanvas, disp: &Chip8Display) {
    let mut x = 0;
    let mut y = 0;
    canvas.set_draw_color(Color::RGB(100, 225, 0));
    for row in disp.pixels {
        x = 0;
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

fn main() {
    let my_disp = Chip8Display {
        pixels: [[0xF3; 8]; 32]
    };
    open_window(&my_disp);
}
