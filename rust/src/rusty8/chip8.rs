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

struct Chip8Emulator {
	/// 4K memory:
	///
	/// 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
	/// 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
	/// 0x200-0xFFF - Program ROM and work RAM
	mem: [u8; 4096],		
	/// 16 registers. 0-14 general purpose. 15 carry flag
	regs: 	[u8; 16],
	/// Index register
	i: 		u8,
	/// Program counter
	pc: 	u8,
	/// Delay timer
	delay_t:	u8,
	/// sound timer
	sound_t: 	u8,
	/// Stack
	stack: 	[u16; 16],
	/// Stack pointer
	sp: 	u16,
}

impl Chip8Emulator {
    
/*    fn new() -> Chip8Emulator {
    	
    }*/


}