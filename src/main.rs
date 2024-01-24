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


fn main() {
    let my_display = Chip8Display {
        pixels: [[0xF0; 8]; 32]
    };
    my_display.draw();
}
