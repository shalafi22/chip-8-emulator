use std::{fs::File, io::{BufReader, Error, Read}, thread, time::Duration};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, EventPump};
use sdl2::rect;
use sdl2::render::WindowCanvas;

use crate::instruction;



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
    pub display: Chip8Display,

    //Canvas
    pub canvas: WindowCanvas,
}

#[derive(Copy, Clone)]
pub enum Pixel {
    On,
    Off
}

impl Pixel {
    fn is_on(&self) -> bool {
        match self {
            Pixel::On => true,
            Pixel::Off => false
        }
    }
}

pub struct Chip8Display {
    pub pixels: [[Pixel; 64];32]
}

impl Chip8Display {
    pub fn clear(&mut self) {
        self.pixels = [[Pixel::Off; 64]; 32];
    }
}

impl Chip8 {
    pub fn new_default(canvas: WindowCanvas) -> Chip8 {
        let memory: [u8; 4096] = [0; 4096];
        let Vx: [u8; 16] = [0; 16];
        let I: u16 = 0;
        let delay_timer: u8 = 0;
        let sound_timer: u8 = 0;
        let PC: u16 = 0;
        let SP: u8 = 0;
        let stack: [u16; 16] = [0; 16];
        let display: Chip8Display = Chip8Display { pixels: [[Pixel::Off; 64]; 32] };
        let mut chip = Chip8 {
            memory, Vx, I, delay_timer, sound_timer, PC, SP, stack, display, canvas
        };
        let mut location = 0x50;
        for sprite in SPRITE_PRESET {
            chip.load_sprite(sprite, location);
            location += 5;
        }
        chip
    }

    fn load_sprite(&mut self, sprite: Sprite,mut location: usize) {
        let mut i: usize = 0;
        while i < sprite.size {
            self.memory[location] = sprite.bytes[i];
            i += 1;
            location += 1;
        }
    }


    pub fn draw_sprite_in_mem_to_x_y(&mut self, sprite_loc: usize, mut x: usize, mut y: usize, n: usize) {
        let mut current_byte = self.memory[sprite_loc];
        let starting_x = x;
        for i in 0..n {
            current_byte = self.memory[sprite_loc + i];
            x = starting_x;
            let mut mask = 0b10000000;
            while mask != 0 {
                if current_byte & mask == mask {
                    self.display.pixels[y][x % 64] = match self.display.pixels[y][x % 64] {
                        Pixel::Off => Pixel::On,
                        Pixel::On => Pixel::Off
                    } 
                }
                mask = mask >> 1;
                x += 1;
            }
            y += 1;
        }

        self.draw_display_to_window();
    }


    /// loads .ch8 file under roms to the memory of the emulator
    pub fn load_file_to_mem(&mut self, filename: &str) -> Result<(), Error> {
        let filename = String::from("./roms/") + filename;
        let f = match File::open(filename) {
            Ok(f) => {f},
            Err(e) => {return Err(e)}
        };
        let reader = BufReader::new(f);
        let reader_bytes = reader.bytes();
        let mut location: usize = 0x200;
        for byte in reader_bytes {
            match byte {
                Ok(b) => {
                    self.memory[location] = b;
                    location += 1;
                },
                Err(e) => println!("Error reading file: {}", e)
            }
        }
        Ok(())
    }

    pub fn get_mem_state(&self) {
        println!("Mem state:");
        println!("-------------------------------------------");
        for i in 0..8 {
            for j in (512 * i)..(512 * i + 511) {
                print!("{} ", &self.memory[j]);
            }
            print!("\n");
        }
        println!("-------------------------------------------");
    }

    pub fn get_reg_state(&self) {
        println!("-------------------------------------------");
        for i in 0..16 {
            println!("V{}: {}", i, &self.Vx[i]);
        }
        println!("I: {:#04x}", &self.I);
        println!("DT: {}", &self.delay_timer);
        println!("ST: {}", &self.sound_timer);
        println!("PC: {:#04x}", &self.PC);
        println!("SP: {}", &self.SP);
        print!("stack: ");
        for i in 0..16 {
            print!("{} ", &self.stack[i]);
        }
        print!("\n");
    }

    pub fn start_device(&mut self, filename: &str, is_debug: bool, event_pump: EventPump) -> Result<(), Error> {
        match self.load_file_to_mem(&filename) {
            Err(e) => return Err(e),
            _ => {}
        };
        self.PC = 0x200;

        if is_debug {
            self.start_debug(event_pump);
        }else {
            self.start_loop(event_pump);
        }
        Ok(())
    }

    fn start_loop(&mut self, mut event_pump: EventPump) {
        'running: loop {
            let instruction: u16 = ((self.memory[self.PC as usize] as u16) << 8) | (self.memory[(self.PC + 1) as usize]) as u16;
            
            self.PC += 2;

            match self.decode_execute_instruction(instruction) {
                instruction::InstructionResult::BreakLoop => break 'running,
                instruction::InstructionResult::Ok => {}
            };
            
            thread::sleep(Duration::new(0, 1_000_000));
        }
        println!("Execution finished, press space to leave");
        'exit: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                        break 'exit;
                    },
                    _ => {}
                }
            }
        }
    }

    fn start_debug(&mut self, mut event_pump: EventPump) {
        'running: loop {
            
            let cur_instruction: u16 = ((self.memory[self.PC as usize] as u16) << 8) | (self.memory[(self.PC + 1) as usize]) as u16;
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                        self.PC += 2;
                        match self.decode_execute_instruction(cur_instruction) {
                            instruction::InstructionResult::BreakLoop => break 'running,
                            instruction::InstructionResult::Ok => {}
                        };
                        println!("Executed instruction: {:#04x}, at mem loc: {:#04x}", cur_instruction, self.PC - 2);
                    },
                    Event::KeyDown { keycode: Some(Keycode::M), .. } => {
                        self.get_mem_state();
                    },
                    Event::KeyDown { keycode: Some(Keycode::Y), .. } => {
                        self.get_reg_state();
                    },
                    Event::KeyDown { keycode: Some(Keycode::K), .. } => {
                        println!("Current instruction: {}, at mem loc: {}", self.memory[self.PC as usize], self.PC);
                    },
                    _ => {}
                }
            }
        }
    }

    /// draws the pixels of Chip8isplay to SDL2 canvas
    /// pixels are represented as 10x10 rectangles
    /// called by the running loop of canvas window
    pub fn draw_display_to_window(&mut self) {
        let mut y = 0;
        self.canvas.set_draw_color(Color::RGB(100, 225, 0));
        for row in self.display.pixels {
            let mut x = 0;
            for pixel in row {
                if pixel.is_on() {
                    let rect = rect::Rect::new(x, y, 10, 10);
                    self.canvas.draw_rect(rect).unwrap();
                    self.canvas.fill_rect(rect).unwrap();
                } 
                x += 10;
            }
            y += 10;
        }
        self.canvas.present();
    }
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





