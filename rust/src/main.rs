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

mod rusty8;

use std::env;
use rusty8::server::{Chip8Server};
use rusty8::chip8::Chip8;
use rusty8::process::Process;
use rusty8::frontend::{TermFrontend, Frontend};

use rusty8::utils;

const DEFAULT_PORT: u16 = 7890;

fn main() {
	let args: Vec<String> = env::args().collect();
	
	// start in standalone mode & render to terminal
	if args.len() == 2 { 
		standalone_mode(&args[1]);
	// start in server mode
	} else { 
    	server_mode();
	}
}

fn standalone_mode(path_to_rom: &String) {
	println!("Loading ROM: {}", path_to_rom);
    let rom: Vec<u8> = utils::read_file(path_to_rom).unwrap();
    println!("Rom loaded: {} bytes", rom.len());
    
    let mut term = TermFrontend::new();
    term.clear();
	let mut process = Process::new(term);
	process.run(rom);
}

fn server_mode() {
    let server = Chip8Server::new("Rusty8 emulation server", DEFAULT_PORT);
	server.start();
}
