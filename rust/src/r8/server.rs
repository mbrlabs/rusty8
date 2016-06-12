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

pub struct Chip8Server {
	name: String,
	port: i32,
}

impl Chip8Server {

	pub fn new(name: &'static str, port: i32) -> Chip8Server {
		return Chip8Server{name: name.to_string(), port: port};
	}

	pub fn start(&self) {
		println!("Server running...");
		loop {
			// TODO accept new requests and spawn thread for emulation
		}
		println!("Server down.");
	}
}
