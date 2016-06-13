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

use rusty8::utils::Stack;

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
    mem:        [u8; MEMORY_SIZE],     
    /// 16 registers. 0-14 general purpose. 
    /// 15th register: carry flag, set if sprite is set from 1 to 0 (collision detection)
    v:          [u8; REGISTER_COUNT],
    /// Index register. Used to store addresses
    i:          u8,
    /// Program counter
    pc:         usize,
    /// Delay timer. Decrements at 60hz if set to a value > 0
    delay:      u8,
    /// Sound timer. Decrements at 60hz if set to a value > 0. As long as > 0, chip-8 will beep.
    sound:      u8,
    /// Stack, used for jump instructions & subroutines
    stack:      Stack,
    /// Set to true if screen must be updated
    draw_flag:  bool,
}

impl Chip8 {
    
    pub fn new() -> Chip8 {
        let mut chip = Chip8 {
            mem: [0; MEMORY_SIZE], v: [0; REGISTER_COUNT],
            i: 0, pc: 0x200, delay: 0, sound: 0, 
            stack: Stack::new(), draw_flag: false,
        };

        // load font
        for i in 0..FONT.len() {
            chip.mem[80 + i] = FONT[i];
        }

        return chip;
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        // TODO implement
    }

    pub fn load_rom_from_file(&mut self, path: &String) {

    }

    pub fn tick(&mut self) {
        // fetch
        let opcode: u16 = (self.mem[self.pc] as u16) << 8 | (self.mem[self.pc + 1] as u16);

        // decode && execute
        match opcode & 0xF000 {
            0x0 => {
                match opcode & 0xFF00 {
                    0x00E0 => {},
                    0x00EE => {},
                    _      => {/* 0x0NNN may be a candidate here */}
                }
            },
            0x1 => {},
            0x2 => {},
            0x3 => {},
            0x4 => {},
            0x5 => {},
            0x6 => {},
            0x7 => {},
            0x8 => {},
            0x9 => {},
            0xA => {},
            0xB => {},
            0xC => {},
            0xD => {},
            0xE => {},
            0xF => {},
            _   => {},
        }

        // update timer registers
    }

    pub fn draw_requested(&self) -> bool {
        return self.draw_flag;
    }


}