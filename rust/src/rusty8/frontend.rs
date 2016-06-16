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

use std::net::TcpStream;
use rusty8::chip8::Chip8;

pub trait Frontend {
    fn clear(&self);
    fn render(&self, chip: &Chip8);
    fn do_input(&self, chip: &mut Chip8);
} 

pub struct TermFrontend {
    // TODO
}

impl TermFrontend {
    pub fn new() -> TermFrontend {
        return TermFrontend{};
    }
}

impl Frontend for TermFrontend {
    fn clear(&self) {
        println!("\x1b[2J\x1b[H");
    }

    fn render(&self, chip: &Chip8) {
        
    }

    fn do_input(&self, chip: &mut Chip8) {

    }
}

pub struct RemoteFrontend {
    stream: TcpStream,
}

impl RemoteFrontend {
    pub fn new(stream: TcpStream) -> RemoteFrontend {
        return RemoteFrontend{stream: stream};
    }
}

impl Frontend for RemoteFrontend {
    fn clear(&self) {

    }

    fn render(&self, chip: &Chip8) {

    }
    
    fn do_input(&self, chip: &mut Chip8) {

    }
}

