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
use rusty8::frontend::Frontend;
use std::io::{Read, Write};

pub struct Process<T: Frontend> {
    /// Frontend, used for rendering & input
    frontend:   T,
    /// emulator
    chip8:  Chip8,
}

impl<T: Frontend> Process<T> {

    pub fn new(frontend: T) -> Process<T> {
        let process = Process {
            frontend:   frontend,
            chip8:  Chip8::new(),
        };

        return process;
    }

    pub fn run(&mut self) {
        loop {
            self.frontend.do_input(&mut self.chip8);
            self.chip8.tick();
            if self.chip8.draw_requested() {
                self.frontend.render(&self.chip8);
            }            
        }

        //let _ = self.conn.control.write(&[65, 76]);
        // this blocks until the the client writes something
        //let _ = self.conn.control.read(&mut [0; 128]);
    }

}