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

    //Display
    pub display: Chip8Display
}

pub struct Chip8Display {
    pub pixels: [[u8; 8];32]
}

pub struct Sprite {
    bytes: [u8; 15],
    size: usize
}

const ZERO_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x90, 0x90, 0x90, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const ONE_SPRITE: Sprite = Sprite {bytes: [0x20, 0x60, 0x20, 0x20, 0x70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const TWO_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x10, 0xF0, 0x80, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const THREE_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x10, 0xF0, 0x10, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const FOUR_SPRITE: Sprite = Sprite {bytes: [0x90, 0x90, 0xF0, 0x10, 0x10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const FIVE_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x80, 0xF0, 0x10, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const SIX_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x80, 0xF0, 0x90, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const SEVEN_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x10, 0x20, 0x40, 0x40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const EIGHT_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x90, 0xF0, 0x90, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const NINE_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x90, 0xF0, 0x10, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const A_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x90, 0xF0, 0x90, 0x90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const B_SPRITE: Sprite = Sprite {bytes: [0xE0, 0x90, 0xE0, 0x90, 0xE0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const C_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x80, 0x80, 0x80, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const D_SPRITE: Sprite = Sprite {bytes: [0xE0, 0x90, 0x90, 0x90, 0xE0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const E_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x80, 0xF0, 0x80, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const F_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x80, 0xF0, 0x80, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};

const SPRITE_PRESET: [Sprite; 16] = [ZERO_SPRITE, ONE_SPRITE, TWO_SPRITE, THREE_SPRITE, FOUR_SPRITE, FIVE_SPRITE, SIX_SPRITE, SEVEN_SPRITE,
                                    EIGHT_SPRITE, NINE_SPRITE, A_SPRITE, B_SPRITE, C_SPRITE, D_SPRITE, E_SPRITE, F_SPRITE];

impl Chip8 {
    pub fn new_default() -> Chip8 {
        let memory: [u8; 4096] = [0; 4096];
        let Vx: [u8; 16] = [0; 16];
        let I: u16 = 0;
        let delay_timer: u8 = 0;
        let sound_timer: u8 = 0;
        let PC: u16 = 0;
        let SP: u8 = 0;
        let stack: [u16; 16] = [0; 16];
        let display: Chip8Display = Chip8Display { pixels: [[0; 8]; 32] };
        let mut chip = Chip8 {
            memory, Vx, I, delay_timer, sound_timer, PC, SP, stack, display
        };
        let mut location = 0x50;
        for sprite in SPRITE_PRESET {
            chip.load_sprite(sprite, location);
            location += 1;
        }
        chip
    }

    fn load_sprite(&mut self, sprite: Sprite, location: usize) {
        let mut i: usize = 0;
        while i < sprite.size {
            self.memory[location] = sprite.bytes[i];
            i += 1;
        }
    }

    pub fn turn_on_display(&self) {
        open_window(&self.display);
    }
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

        //60 FPS        
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
    let mut my_chip8 = Chip8::new_default();
    my_chip8.turn_on_display();

    println!("DEBUG: 0x50: {}", my_chip8.memory[0x50]);
    println!("DEBUG: 0x53: {}", my_chip8.memory[0x53]);
    println!("DEBUG: 0x54: {}", my_chip8.memory[0x54]);
    println!("DEBUG: 0x56: {}", my_chip8.memory[0x56]);
}
