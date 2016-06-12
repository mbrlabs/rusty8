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
// along with rusty8.  If not, see <http://www.gnu.org/licenses/>.

use std::net::{TcpStream};
use std::io::{Write, Read};

pub struct Chip8Process {
    /// client -> server: used for sending the rom & user input
    control_stream: TcpStream,
    // server -> client: used for sending rendering data 
    //data_stream: TcpStream,
    // server -> client: used for debugging
    //debug_stream: TcpStream,
}

impl Chip8Process {

    pub fn new(control_stream: TcpStream) -> Chip8Process {
        // TODO establish 2 more tcp connections:
        // 1. data connection for sending render data
        // 2. dubug connection for emulator debugging

        let process = Chip8Process{
            control_stream: control_stream,
        };

        return process;
    }

    pub fn execute(&mut self) {
        // TODO 1. receive rom
        // TODO 2. start emulating
        // TODO 3. receive input commands & apply them
        // TODO 4. send rendering data to client
        
        let _ = self.control_stream.write(&[65, 76]);
     //   let _ = self.control_stream.read(&mut [0; 128]);
    }

}