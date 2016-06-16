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
use std::io::{Read, Write};
use std::thread;

use rusty8::process::Process;
use rusty8::frontend::{RemoteFrontend, Frontend};

pub struct Chip8Server {
    name: String,
    port: u16,
}

// TODO this is NOT tested and will fail to 99.999% ;)
fn handle_request(mut stream: TcpStream) {
    println!("New client connected. Reading rom...");
    let mut buf: [u8; 2] = [0; 2];
    let mut bytes_read = stream.read(&mut buf).unwrap();

    if bytes_read != 2 {
        println!("Error while reading the rom");
        drop(stream);
        return;
    } 

    // read rom size
    let mut rom_size: usize = ((buf[0] as u16) << 8 | buf[1] as u16) as usize;
    println!("Rom size sent form client: {}", rom_size);
    // read rom
    let mut rom: Vec<u8> = Vec::new();
    let mut buf: [u8; 1024] = [0; 1024];
    while rom_size > 0 {
        bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read <= rom_size {
            rom_size -= bytes_read;
            rom.extend_from_slice(&buf[0..bytes_read]); // TODO may not work
        } else {
            println!("Rom size exeeds then described size in header");
            drop(stream);
            return;
        }
    } 

    println!("Received rom with {} bytes.", rom.len());

    // create process and start emulating
    let remote = RemoteFrontend::new(stream);
    let mut proccess = Process::new(remote);
    proccess.run(rom);
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
                        handle_request(stream);
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