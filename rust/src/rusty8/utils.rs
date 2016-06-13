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

use rusty8::chip8::Chip8;

const SIZE: usize = 16;

pub struct Stack {
    data: [u16; 16],
    ptr:  usize,
}

impl Stack {
    
    pub fn new() -> Stack {
        return Stack{data: [0; SIZE], ptr: 0};
    }

    pub fn push(&mut self, n: u16) {
        if self.ptr == SIZE - 1 { 
            panic!("Stack overflow!"); 
        }
        self.ptr += 1;
        self.data[self.ptr] = n;
    }

    pub fn pop(&mut self) -> u16 {
        if self.ptr == 0 { 
            panic!("Stack underflow!"); 
        }
        self.ptr -= 1;
        return self.data[self.ptr + 1];
    }

    pub fn peek(&self) -> u16 {
        return self.data[self.ptr];
    }
}

pub fn render_to_term(chip: &Chip8) {

}