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
use std::io::{Read, Write};
use rusty8::chip8::Chip8;

pub trait Frontend {
    fn clear(&mut self);
    fn render(&mut self, chip: &Chip8);
    fn do_input(&mut self, chip: &mut Chip8);
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
    fn clear(&mut self) {
        println!("\x1b[2J\x1b[H");
    }

    fn render(&mut self, chip: &Chip8) { 
    }

    fn do_input(&mut self, chip: &mut Chip8) {

    }
}

pub struct RemoteFrontend {
    stream: TcpStream,
    keys:   [bool; 16],
}

impl RemoteFrontend {
    pub fn new(stream: TcpStream) -> RemoteFrontend {
        return RemoteFrontend{stream: stream, keys: [false; 16]};
    }
}

impl Frontend for RemoteFrontend {
    fn clear(&mut self) {
        let _ = self.stream.write(&[0]);
    }

    fn render(&mut self, chip: &Chip8) {
        let mut buf: [u8; 2048] = [0; 2048];
        for i in 0..buf.len() {
            if chip.vram[i] {
                buf[i] = 1;
            }
        } 
        let written = self.stream.write(&buf).unwrap();
    }
    
    fn do_input(&mut self, chip: &mut Chip8) {
        let mut buf: [u8; 16] = [0; 16];
        let bytes_read = self.stream.read(&mut buf).unwrap();
        let slice = &buf[0..bytes_read];

        let mut i = 0;
        for i in 0..slice.len() {
            self.keys[i as usize] = slice[i] >= 1;
            if slice[i] >= 1 {
                println!("Key pressed: {}", i);
            }
        }

        chip.set_keys(self.keys);
    }
}

