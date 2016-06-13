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

use rusty8::server::Rusty8Connection;
use rusty8::chip8::Chip8;

use std::io::{Read, Write};

pub struct Chip8Process {
    /// rusty8 connection 
    conn:   Rusty8Connection,
    /// emulator
    chip8:  Chip8,
}

impl Chip8Process {

    pub fn new(conn: Rusty8Connection) -> Chip8Process {
        let process = Chip8Process{
            conn:   conn,
            chip8:  Chip8::new(),
        };

        return process;
    }

    pub fn execute(&mut self) {
        // TODO 1. receive rom
        // TODO 2. start emulating
        // TODO 3. receive input commands & apply them
        // TODO 4. send rendering data to client
        
        let _ = self.conn.control.write(&[65, 76]);
        // this blocks until the the client writes something
        let _ = self.conn.control.read(&mut [0; 128]);
    }

}