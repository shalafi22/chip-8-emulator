

use crate::Chip8;
use rand::Rng;


pub enum InstructionResult {
    BreakLoop,
    StartDelayTimer,
    StartSoundTimer,
    SkipIfPressed(u8),
    SkipIfNotPressed(u8),
    WaitForKey(u8),
    Ok
}

impl Chip8 {
    pub fn decode_execute_instruction(&mut self, instruction: u16) -> InstructionResult {
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
                    return InstructionResult::BreakLoop;
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
                self.Vx[((instruction & 0x0F00) >> 8) as usize] = u8::wrapping_add(self.Vx[((instruction & 0x0F00) >> 8) as usize], (instruction & 0x00FF) as u8);
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
                        
                        self.Vx[((instruction & 0x0F00) >> 8) as usize] = u8::wrapping_sub(self.Vx[((instruction & 0x0F00) >> 8) as usize], self.Vx[((instruction & 0x00F0) >> 4) as usize]);
                    },
                    0x0006 => {
                        //8xy6
                        //Vf = Vx & 0x0001, Vx = Vx >> 1
                        self.Vx[0x000F] = (self.Vx[((instruction & 0x0F00) >> 8) as usize] & 0x0001) as u8;
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
                        self.Vx[((instruction & 0x00F0) >> 4) as usize] = u8::wrapping_sub(self.Vx[((instruction & 0x00F0) >> 8) as usize], self.Vx[((instruction & 0x0F00) >> 8) as usize]);
                    },
                    0x000E => {
                        //8xyE
                        //Vf = Vx & 0x1000, Vx = Vx << 1
                        self.Vx[0x000F] = (self.Vx[((instruction & 0x0F00) >> 8) as usize] & 0x80) >> 7;
                        self.Vx[((instruction & 0x0F00) >> 8) as usize] = self.Vx[((instruction & 0x0F00) >> 8) as usize] << 1; 
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
                    return InstructionResult::SkipIfPressed(((instruction & 0x0F00) >> 8) as u8);
                } else if instruction & 0x00A1 == 0x00A1 {
                    //ExA1
                    //Skip next instruction if key with value Vx is not pressed
                    return InstructionResult::SkipIfNotPressed(((instruction & 0x0F00) >> 8) as u8);
                } else {
                    println!("Invalid instruction at mem: {}, {:#04x}", self.PC, instruction)
                }
            },
            0xF000 => {
                if instruction & 0x00FF == 0x0007 {
                    //Fx07
                    //set  Vx = delay timer
                    self.Vx[((instruction & 0x0F00) >> 8) as usize] = self.delay_timer;
                } else if instruction & 0x00FF == 0x000A {
                    // Wait for a key press, store the value of the key in Vx.
                    // All execution stops until a key is pressed, then the value of that key is stored in Vx.
                    return InstructionResult::WaitForKey(((instruction & 0x0F00) >> 8) as u8);
                } else if instruction & 0x00FF == 0x0015 {
                    // Fx15
                    // set DT = Vx
                    self.delay_timer = self.Vx[((instruction & 0x0F00) >> 8) as usize];
                    return InstructionResult::StartDelayTimer;
                } else if instruction & 0x00FF == 0x0018 {
                    // Fx18
                    // Set sound timer = Vx
                    self.sound_timer = self.Vx[((instruction & 0x0F00) >> 8) as usize];
                    return InstructionResult::StartSoundTimer;
                } else if instruction & 0x00FF == 0x001E {
                    //Fx1E
                    //set I += Vx
                    self.I += self.Vx[((instruction & 0x0F00) >> 8) as usize] as u16;
                } else if instruction & 0x00FF == 0x0029 {
                    //Fx29
                    // I = location of sprite for hexadecimal x
                    self.I = 0x50 + (5 * ((instruction & 0x0F00) >> 8));
                } else if instruction & 0x00FF == 0x0033 {
                    //Fx33
                    //Store BCD representation of Vx in memory locations I, I+1, and I+2.
                    let num = self.Vx[((instruction & 0x0F00) >> 8) as usize];
                    self.memory[self.I as usize] = num / 100;
                    self.memory[(self.I + 1) as usize] = (num / 10) % 10;
                    self.memory[(self.I + 2) as usize] = num % 10;
                } else if instruction & 0x00FF == 0x0055 {
                    //Fx55
                    //Store registers V0 through Vx in memory starting at location I
                    for i in 0..=((instruction & 0x0F00) >> 8) {
                        self.memory[(self.I + i) as usize] = self.Vx[i as usize];
                    }
                } else if instruction & 0x00FF == 0x0065 {
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
        InstructionResult::Ok
    }
}