use std::{fs::File, io::{BufReader, Error, Read}, iter::Inspect};
use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::render::WindowCanvas;
use rand::Rng;


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

//TODO: pixels represented as enum with variants On, Off.
//      Display is [[Pixel; 64]; 32]
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
    pub fn load_file_to_mem(&mut self) -> Result<(), Error> {
        //TODO: get filename from user
        let filename = "./roms/maze.ch8";
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
        println!("I: {}", &self.I);
        println!("DT: {}", &self.delay_timer);
        println!("ST: {}", &self.sound_timer);
        println!("PC: {}", &self.PC);
        println!("SP: {}", &self.SP);
        print!("stack: ");
        for i in 0..16 {
            print!("{} ", &self.stack[i]);
        }
        print!("\n");
    }

    pub fn start_device(&mut self) {
        self.PC = 0x200;
        //TODO: make this a loop
        'running: loop {
            let instruction: u16 = ((self.memory[self.PC as usize] as u16) << 8) | (self.memory[(self.PC + 1) as usize]) as u16;
            
            self.PC += 2;

            match instruction & 0xF000 {
                0x0000 => {
                    if instruction == 0x00E0 {
                        //clear display

                        self.display.clear();
                        self.draw_display_to_window();
                    } else if instruction == 0x00EE {
                        //return from subroutine

                        self.PC = self.stack[self.SP as usize];
                        self.SP -= 1;
                    } else {
                        println!("Invalid instruction at mem: {}, {:#04x}", self.PC, instruction)
                    }
                },
                0x1000 => {
                    //set PC to nnn
                    if self.PC - 2 == instruction & 0x0FFF {
                        break 'running;
                    }
                    self.PC = instruction & 0x0FFF;
                },
                0x2000 => {
                    //call subroutine at nnn

                    self.SP += 1;
                    self.stack[self.SP as usize] = self.PC;
                    self.PC = instruction & 0x0FFF;
                },
                0x3000 => {
                    //3xkk
                    //if Vx == kk skip instruction
                    if self.Vx[((instruction & 0x0F00) >> 8) as usize] == ((instruction & 0x00FF) as u8) {
                        self.PC += 2;
                    }
                },
                0x4000 => {
                    //4xkk
                    //if Vx != kk skip instruction
                    if self.Vx[((instruction & 0x0F00) >> 8) as usize] != ((instruction & 0x00FF) as u8) {
                        self.PC += 2;
                    }
                },
                0x5000 => {
                    //5xy0
                    //if Vx == Vy skip instruction
                    if instruction & 0x000F == 0 {
                        if self.Vx[((instruction & 0x0F00) >> 8) as usize] == self.Vx[((instruction & 0x00F0) >> 4) as usize] {
                            self.PC += 2;
                        }
                    }
                },
                0x6000 => {
                    //6xkk
                    //put value kk in register Vx
                    self.Vx[((instruction & 0x0F00) >> 8) as usize] = (instruction & 0x00FF) as u8;
                },
                0x7000 => {
                    //7xkk
                    //set Vx += kk
                    self.Vx[((instruction & 0x0F00) >> 8) as usize] += (instruction & 0x00FF) as u8;
                },
                0x8000 => {
                    match instruction & 0x000F {
                        0x0000 => {
                            //8xy0
                            //store the val of Vy in Vx
                            self.Vx[((instruction & 0x0F00) >> 8) as usize] = self.Vx[((instruction & 0x00F0) >> 4) as usize];
                        },
                        0x0001 => {
                            //8xy1
                            //Vx = Vx | Vy
                            self.Vx[((instruction & 0x0F00) >> 8) as usize] = self.Vx[((instruction & 0x0F00) >> 8) as usize] | self.Vx[((instruction & 0x00F0) >> 4) as usize]; 
                        },
                        0x0002 => {
                            //8xy2
                            //Vx = Vx & Vy
                            self.Vx[((instruction & 0x0F00) >> 8) as usize] = self.Vx[((instruction & 0x0F00) >> 8) as usize] & self.Vx[((instruction & 0x00F0) >> 4) as usize]; 
                        },
                        0x0003 => {
                            //8xy3
                            //Vx = Vx ^ Vy
                            self.Vx[((instruction & 0x0F00) >> 8) as usize] = self.Vx[((instruction & 0x0F00) >> 8) as usize] ^ self.Vx[((instruction & 0x00F0) >> 4) as usize]; 
                        },
                        0x0004 => {
                            //8xy4
                            //Vx = Vx + Vy, VF = carry
                            let (result, of) = self.Vx[((instruction & 0x0F00) >> 8) as usize].overflowing_add(self.Vx[((instruction & 0x00F0)>> 4) as usize]);
                            self.Vx[((instruction & 0x0F00) >> 8) as usize] = result;
                            if of {
                                self.Vx[0xF] = 1;
                            } else {
                                self.Vx[0xF] = 0;
                            }
                        },
                        0x0005 => {
                            //8xy5
                            //if Vx > Vy, Vf = 1, else Vf = 0. Vx = Vx - Vy
                            if self.Vx[((instruction & 0x0F00) >> 8) as usize] > self.Vx[((instruction & 0x00F0) >> 4) as usize] {
                                self.Vx[0x000F] = 1;
                            } else {
                                self.Vx[0x000F] = 0;
                            }
                            self.Vx[((instruction & 0x0F00) >> 8) as usize] -= self.Vx[((instruction & 0x00F0) >> 4) as usize];
                        },
                        0x0006 => {
                            //8xy6
                            //Vf = Vx & 0x0001, Vx = Vx >> 1
                            self.Vx[0x000F] = (instruction & 0x0001) as u8;
                            self.Vx[((instruction & 0x0F00) >> 8) as usize] = self.Vx[((instruction & 0x0F00) >> 8) as usize] >> 1; 
                        },
                        0x0007 => {
                            //8vx7
                            //if Vy > Vx, Vf = 1, else Vf = 0. Vx = Vy - Vx
                            if self.Vx[((instruction & 0x0F00) >> 8) as usize] < self.Vx[((instruction & 0x00F0) >> 4) as usize] {
                                self.Vx[0x000F] = 1;
                            } else {
                                self.Vx[0x000F] = 0;
                            }
                            self.Vx[((instruction & 0x00F0) >> 4) as usize] -= self.Vx[((instruction & 0x0F00) >> 8) as usize];
                        },
                        0x000E => {
                            //8xyE
                            //Vf = Vx & 0x1000, Vx = Vx << 1
                            self.Vx[0x000F] = (self.Vx[((instruction & 0x0F00) >> 8) as usize] & 0x80) >> 7;
                            self.Vx[(instruction & 0x0F00) as usize] = self.Vx[(instruction & 0x0F00) as usize] << 1; 
                        },
                        _ => println!("Invalid instruction at mem: {}, {:#04x}", self.PC, instruction)
                    }
                },
                0x9000 => {
                    //9xy0
                    //Skip next instruction if Vx != Vy.
                    if instruction & 0x0000 == 0 {
                        if self.Vx[((instruction & 0x0F00) >> 8) as usize] != self.Vx[((instruction & 0x00F0) >> 4) as usize] {
                            self.PC += 2;
                        }
                    }
                },
                0xA000 => {
                    //set I to nnn
                    self.I = instruction & 0x0FFF;
                },
                0xB000 => {
                    //The program counter is set to nnn plus the value of V0.
                    self.PC = (instruction & 0x0FFF) + self.Vx[0] as u16; 
                },
                0xC000 => {
                    //Cxkk
                    //Set Vx = random byte AND kk
                    self.Vx[((instruction & 0x0F00) >> 8) as usize] = rand::thread_rng().gen_range(0..=255) & ((instruction & 0x00FF) as u8); 
                },
                0xD000 => {
                    //Dxyn
                    //display the n-long sprite at location I to (Vx, Vy)
                    self.draw_sprite_in_mem_to_x_y(self.I as usize, self.Vx[((instruction & 0x0F00) >> 8) as usize] as usize, self.Vx[((instruction & 0x00F0) >> 4) as usize] as usize, (instruction & 0x000F) as usize);
                },
                0xE000 => {
                    if instruction & 0x009E == 0x009E {
                        //Ex9E
                        //Skip next instruction if key with value Vx is pressed

                    } else if instruction & 0x00A1 == 0x00A1 {
                        //ExA1
                        //Skip next instruction if key with value Vx is not pressed
                    } else {
                        println!("Invalid instruction at mem: {}, {:#04x}", self.PC, instruction)
                    }
                },
                0xF000 => {
                    if instruction & 0x0007 == 0x0007 {
                        //Fx07
                        //set  Vx = delay timer
                        self.Vx[((instruction & 0x0F00) >> 8) as usize] = self.delay_timer;
                    } else if instruction & 0x000A == 0x000A {
                        // Wait for a key press, store the value of the key in Vx.
                        // All execution stops until a key is pressed, then the value of that key is stored in Vx.

                    } else if instruction & 0x0015 == 0x0015 {
                        // Fx15
                        // set DT = Vx
                        self.delay_timer = self.Vx[((instruction & 0x0F00) >> 8) as usize];
                    } else if instruction & 0x0018 == 0x0018 {
                        // Fx18
                        // Set sound timer = Vx
                        self.sound_timer = self.Vx[((instruction & 0x0F00) >> 8) as usize];
                    } else if instruction & 0x001E == 0x001E {
                        //Fx1E
                        //set I += Vx
                        self.I += self.Vx[((instruction & 0x0F00) >> 8) as usize] as u16;
                    } else if instruction & 0x0029 == 0x0029 {
                        //Fx29
                        // I = location of sprite for hexadecimal x
                        self.I = 0x50 + (5 * ((instruction & 0x0F00) >> 8));
                    } else if instruction & 0x0033 == 0x0033 {
                        //Fx33
                        //Store BCD representation of Vx in memory locations I, I+1, and I+2.
                        let num = self.Vx[((instruction & 0x0F00) >> 8) as usize];
                        self.memory[self.I as usize] = num / 100;
                        self.memory[(self.I + 1) as usize] = (num / 10) % 10;
                        self.memory[(self.I + 2) as usize] = num % 10;
                    } else if instruction & 0x0055 == 0x0055 {
                        //Fx55
                        //Store registers V0 through Vx in memory starting at location I
                        for i in 0..=((instruction & 0x0F00) >> 8) {
                            self.memory[(self.I + i) as usize] = self.Vx[i as usize];
                        }
                    } else if instruction & 0x0065 == 0x0065 {
                        //Fx65
                        //Read registers V0 through Vx from memory starting at location I.
                        for i in 0..=((instruction & 0x0F00) >> 8) {
                            self.Vx[i as usize] = self.memory[(self.I + i) as usize];
                        }

                    } else {
                        println!("Invalid instruction at mem: {}, {:#04x}", self.PC, instruction)
                    }
                },
                _ => {println!("Invalid instruction at mem: {}, {:#04x}", self.PC, instruction)}
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





