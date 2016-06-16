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

use std::io::{Read, Write};
use rusty8::chip8::Chip8;

pub trait Frontend {
    fn clear(&mut self);
    fn render(&mut self, chip: &Chip8);
    fn do_input(&mut self, chip: &mut Chip8);
} 

pub struct SdlFrontend {
    // TODO
}

impl SdlFrontend {
    pub fn new() -> SdlFrontend {
        return SdlFrontend{};
    }
}

impl Frontend for SdlFrontend {
    fn clear(&mut self) {
        
    }

    fn render(&mut self, chip: &Chip8) { 
    }

    fn do_input(&mut self, chip: &mut Chip8) {

    }
}