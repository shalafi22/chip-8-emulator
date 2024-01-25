use crate::draw_display_to_window;
use sdl2::render::WindowCanvas;

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

//TODO: pixels represented as enum with variants On, Off.
//      Display is [[Pixel; 64]; 32]
pub struct Chip8Display {
    pub pixels: [[u8; 8];32]
}

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


    pub fn draw_sprite_in_mem_to_x_y(&mut self, mut sprite_loc: usize, x: usize, mut y: usize, mut n: usize, canvas: &mut WindowCanvas) {
        while n > 0 {
            self.display.pixels[y][x % 8] = self.memory[sprite_loc];
            n -= 1;
            sprite_loc += 1;
            y += 1;
        }

        draw_display_to_window(canvas, &self.display);
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





