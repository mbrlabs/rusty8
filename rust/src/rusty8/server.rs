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

use std::net::{TcpListener, TcpStream};
use std::thread;

use rusty8::process::Chip8Process;

pub struct Chip8Server {
    name: String,
    port: u16,
}

impl Chip8Server {

    pub fn new(name: &'static str, port: u16) -> Chip8Server {
        return Chip8Server{name: name.to_string(), port: port};
    }

    pub fn start(&self) {
        let addr = ("127.0.0.1", self.port);
        let listener = TcpListener::bind(addr).unwrap();
        println!("'{}' running on port {}", self.name, self.port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move|| {
                        println!("New client connected");
                        let conn = Rusty8Connection{control: stream};
                        let mut proccess = Chip8Process::new(conn);
                        proccess.execute();
                    });             
                }, 
                Err(e) => {
                    // TODO handle failed connection
                    println!("Error");
                }
            }
        }

        drop(listener);
        println!("Shutting down server");
    }
}

pub struct Rusty8Connection {
    /// client -> server: used for sending the rom & user input
    pub control: TcpStream,
    // server -> client: used for sending rendering data 
    //data_stream: TcpStream,
    // server -> client: used for debugging
    //debug_stream: TcpStream,
}

impl Rusty8Connection {
    // TODO implement communication protocol here
}