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

use std::io;
use std::io::prelude::*;
use std::fs::File;

const SIZE: usize = 16;

pub struct Stack {
    data: [usize; 16],
    ptr:  usize,
}

impl Stack {
    
    pub fn new() -> Stack {
        return Stack{data: [0; SIZE], ptr: 0};
    }

    pub fn push(&mut self, n: usize) {
        if self.ptr == SIZE - 1 { 
            panic!("Stack overflow!"); 
        }
        self.ptr += 1;
        self.data[self.ptr] = n;
    }

    pub fn pop(&mut self) -> usize {
        if self.ptr == 0 { 
            panic!("Stack underflow!"); 
        }
        self.ptr -= 1;
        return self.data[self.ptr + 1];
    }

    pub fn peek(&self) -> usize {
        return self.data[self.ptr];
    }

    pub fn clear(&mut self) {
        self.ptr = 0; 
    }
    
}

pub fn read_file(path: &String) -> Result<Vec<u8>, io::Error> {
    let mut data: Vec<u8> = Vec::new();
    let mut file = try!(File::open(path));
    file.read_to_end(&mut data);

    return Ok(data);
}