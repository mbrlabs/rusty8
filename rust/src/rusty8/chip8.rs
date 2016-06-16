// This file is part of rusty8.
// 
// rusty8 is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// any later version.
// 
// rusty8 is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with rusty8. If not, see <http://www.gnu.org/licenses/>.

extern crate rand;

use rusty8::utils;
use rusty8::utils::Stack;


/// Built in font set; starts at mem address 0x50 (80)
const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

const MEMORY_SIZE: usize = 4096;
const REGISTER_COUNT: usize = 16;

pub struct Chip8 {
    /// 4K memory:
    ///
    /// 0x000-0x1FF - Chip 8 interpreter. Just leave this empty..
    /// 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    /// 0x200-0xFFF - Program ROM and work RAM
    pub mem:        [u8; MEMORY_SIZE],     
    /// 16 registers. 0-14 general purpose. 
    /// 15th register: carry flag, set if sprite is set from 1 to 0 (collision detection)
    v:          [u8; REGISTER_COUNT],
    /// Index register. Used to store addresses
    i:          u16,
    /// Program counter
    pc:         usize,
    /// Delay timer. Decrements at 60hz if set to a value > 0
    delay:      u8,
    /// Sound timer. Decrements at 60hz if set to a value > 0. As long as > 0, chip-8 will beep.
    sound:      u8,
    /// Stores state of key; if true => pressed
    keys:       [bool; 16],
    /// Stack, used for jump instructions & subroutines
    stack:      Stack,
    /// Set to true if screen must be updated
    draw_flag:  bool,
    /// Set to true if the screen needs to be cleared
    clear_flag:  bool,
}

impl Chip8 {
    
    pub fn new() -> Chip8 {
        let mut chip = Chip8 {
            mem: [0; MEMORY_SIZE], v: [0; REGISTER_COUNT],
            i: 0, pc: 0x200, delay: 0, sound: 0, keys: [false; 16],
            stack: Stack::new(), draw_flag: false, clear_flag: false,
        };

        // load font
        for i in 0..FONT.len() {
            chip.mem[80 + i] = FONT[i];
        }

        return chip;
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        if rom.len() > 3000 {
            panic!("Rom to big: {} bytes", rom.len());
        }

        // According to this: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1
        // most programs start at 0x200
        for i in 0..rom.len() {
            self.mem[0x200 + i] = rom[i];
        }
    }

    pub fn reset(&mut self) {
        self.stack.clear();
        self.pc = 0x200;
        self.i = 0;
        self.delay = 0;
        self.sound = 0;
        for i in 0..self.mem.len() {
            self.mem[i] = 0;
        }
        for i in 0..self.v.len() {
            self.v[i] = 0;
        }
        self.draw_flag = false;
    }

    pub fn tick(&mut self) {
        // fetch
        let opcode: u16 = (self.mem[self.pc] as u16) << 8 | (self.mem[self.pc + 1] as u16);

        // decode && execute
        // Opcode list: https://en.wikipedia.org/wiki/CHIP-8#Opcode_table
        match opcode & 0xF000 {
            0x0 => {
                match opcode {
                    0x00E0 => {
                        // 00E0: Clears the screen
                        self.clear_flag = true;
                        self.pc += 2;
                    },
                    0x00EE => {
                        // 00EE: Returns from a subroutine
                        // TODO implement
                    },
                    _      => {
                        // 0x0NNN: Calls RCA 1802 program at address NNN. 
                        // Not necessary for most ROMs
                        // TODO implement
                    }
                }
            },
            0x1 => {
                // 1NNN: Jumps to address NNN
                // TODO implement
            },
            0x2 => {
                // 2NNN: Calls subroutine at NNN
                // TODO implement
            },
            0x3 => {
                // 3XNN: Skips the next instruction if VX equals NN
                let vx = (opcode & 0x0F00) >> 8;
                let nn = (opcode & 0x00FF) as u8;
                if self.v[vx as usize] == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x4 => {
                // 4XNN: Skips the next instruction if VX doesn't equal NN
                let vx = (opcode & 0x0F00) >> 8;
                let nn = (opcode & 0x00FF) as u8;
                if self.v[vx as usize] != nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x5 => {
                // 5XY0: Skips the next instruction if VX equals VY
                let vx = (opcode & 0x0F00) >> 8;
                let vy = (opcode & 0x00F0) >> 4;
                if self.v[vx as usize] == self.v[vy as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x6 => {
                // 6XNN: Sets VX to NN
                let register = (opcode & 0x0F00) >> 8;
                self.v[register as usize] = (opcode & 0x00FF) as u8;
                self.pc += 2;
            },
            0x7 => {
                // 7XNN: Adds NN to VX
                let register = (opcode & 0x0F00) >> 8;
                self.v[register as usize] += (opcode & 0x00FF) as u8;;
                self.pc += 2;
            },
            0x8 => {
                match opcode & 0x000F {
                    0x0000 => {
                        // 8XY0: Sets VX to the value of VY
                        let vx = opcode & 0x0F00 >> 8;
                        let vy = opcode & 0x00F0 >> 4;
                        self.v[vx as usize] = self.v[vy as usize];
                        self.pc += 2;
                    },
                    0x0001 => {
                        // 8XY1: Sets VX to VX or VY
                        let vx = opcode & 0x0F00 >> 8;
                        let vy = opcode & 0x00F0 >> 4;
                        self.v[vx as usize] |= self.v[vy as usize];
                        self.pc += 2;
                    }, 
                    0x0002 => {
                        // 8XY2: Sets VX to VX and VY
                        let vx = opcode & 0x0F00 >> 8;
                        let vy = opcode & 0x00F0 >> 4;
                        self.v[vx as usize] &= self.v[vy as usize];
                        self.pc += 2;
                    }, 
                    0x0003 => {
                        // 8XY3: Sets VX to VX xor VY
                        let vx = opcode & 0x0F00 >> 8;
                        let vy = opcode & 0x00F0 >> 4;
                        self.v[vx as usize] ^= self.v[vy as usize];
                        self.pc += 2;
                    }, 
                    0x0004 => {
                        // 8XY4: Adds VY to VX. VF is set to 1 when there's a carry, 
                        // and to 0 when there isn't
                        // TODO implement
                    },
                    0x0005 => {
                        // 8XY5: VY is subtracted from VX. VF is set to 0 when there's 
                        // a borrow, and 1 when there isn't
                        // TODO implement
                    }, 
                    0x0006 => {
                        // 8XY6: Shifts VX right by one. VF is set to the value of the 
                        // least significant bit of VX before the shift
                        // TODO implement
                    }, 
                    0x0007 => {
                        // 8XY7: Sets VX to VY minus VX. VF is set to 0 when there's a 
                        // borrow, and 1 when there isn't
                        // TODO implement
                    }, 
                    0x000E => {
                        // 8XYE: Shifts VX left by one. VF is set to the value of the 
                        // most significant bit of VX before the shift
                        // TODO implement
                    }, 
                    _      => {/* Unsupported opcode */}
                }
            },
            0x9 => {
                // 9XY0: Skips the next instruction if VX doesn't equal VY
                // TODO implement 
                let vx = (opcode & 0x0F00) >> 8;
                let vy = (opcode & 0x00F0) >> 4;
                if self.v[vx as usize] != self.v[vy as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0xA => {
                // ANNN: Sets I to the address NNN
                self.i = opcode & 0x0FFF;
                self.pc += 2;
            },
            0xB => {
                // BNNN: Jumps to the address NNN plus V0
                // TODO implement
            },
            0xC => {
                // CXNN: Sets VX to the result of a bitwise and operation on a 
                // random number and NN
                let register = (opcode & 0x0F00) >> 8;
                let num = rand::random::<u8>() & (opcode & 0x00FF) as u8;
                self.v[register as usize] = num;
                self.pc += 2;
            },
            0xD => {
                // DXYN: Sprites stored in memory at location in index register (I), 8bits 
                // wide. Wraps around the screen. If when drawn, clears a pixel, register 
                // VF is set to 1 otherwise it is zero. All drawing is XOR drawing 
                // (i.e. it toggles the screen pixels). Sprites are drawn starting at position VX, VY. 
                // N is the number of 8bit rows that need to be drawn. If N is greater than 1, second 
                // line continues at position VX, VY+1, and so on
                // TODO implement
            },
            0xE => {
                match opcode & 0x00FF {
                    0x009E => {
                        // EX9E: Skips the next instruction if the key stored in VX is pressed
                        // TODO implement
                    },
                    0x00A1 => {
                        // EXA1: Skips the next instruction if the key stored in VX isn't pressed
                        // TODO implement
                    },
                    _      => {/* Unsupported opcode */}
                }
            },
            0xF => {
                match opcode & 0x00FF {
                    0x0007 => {
                        // FX07: Sets VX to the value of the delay timer
                        let vx = (opcode & 0x0F00) >> 8;
                        self.v[vx as usize] = self.delay;
                        self.pc += 2;
                    }, 
                    0x000A => {
                        // FX0A: A key press is awaited, and then stored in VX
                        // TODO implement
                    }, 
                    0x0015 => {
                        // FX15: Sets the delay timer to VX
                        let vx = (opcode & 0x0F00) >> 8;
                        self.delay = self.v[vx as usize];
                        self.pc += 2;
                    },
                    0x0018 => {
                        // FX18: Sets the sound timer to VX
                        let vx = (opcode & 0x0F00) >> 8;
                        self.sound = self.v[vx as usize];
                        self.pc += 2;
                    },
                    0x001E => {
                        // FX1E: Adds VX to I
                        let vx = (opcode & 0x0F00) >> 8;
                        self.i += self.v[vx as usize];
                        self.pc += 2;
                    },
                    0x0029 => {
                        // FX29: Sets I to the location of the sprite for the character in VX. 
                        // Characters 0-F (in hexadecimal) are represented by a 4x5 font
                        // TODO implement
                    },
                    0x0033 => {
                        // FX33: Stores the binary-coded decimal representation of VX, with 
                        // the most significant of three digits at the address in I, the middle 
                        // digit at I plus 1, and the least significant digit at I plus 2. 
                        // (In other words, take the decimal representation of VX, place the hundreds 
                        // digit in memory at location in I, the tens digit at location I+1, and the 
                        // ones digit at location I+2.)
                        // TODO implement
                    },
                    0x0055 => {
                        // FX55: Stores V0 to VX (including VX) in memory starting at address I
                        // TODO implement
                    },
                    0x0065 => {
                        // FX65: Fills V0 to VX (including VX) with values from memory starting at address I
                        // TODO implement
                    },
                    _      => {/* Unsupported opcode */}
                }
            },
            _   => {/* Unsupported opcode */},
        }

        // TODO update timer registers
    }

    pub fn draw_requested(&self) -> bool {
        return self.draw_flag;
    }

    pub fn clear_requested(&self) -> bool {
        return self.clear_flag;
    }

}