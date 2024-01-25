use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
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

impl Chip8Display {
    pub fn draw(&self) {
        for row in self.pixels {
            for byte in row {
                let mut mask = 0b10000000;
                for _i in 0..8 {
                    if byte & mask == mask {
                        print!("*");
                    } else {
                        print!(" ");
                    }
                    mask = mask  >> 1;
                }
            }
            print!("\n");
        }
    }
}

fn open_window() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let rect = rect::Rect::new(10, 10, 20, 20);
    canvas.set_draw_color(Color::RGB(60, 65, 44));
    canvas.draw_rect(rect).unwrap();
    canvas.fill_rect(rect).unwrap();
    canvas.present();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


fn main() {
    open_window();
}
