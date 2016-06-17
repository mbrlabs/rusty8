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
extern crate piston_window;
mod rusty8;

use std::env;
use rusty8::chip8::Chip8;
use std::thread;

use rusty8::utils;

use piston_window::*;

const SCALE: f64 = 8.0;


fn main() {

	// load rom
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 { 
		panic!("DUruuuuuuuuuuuuuurrrrrrrrrrrr");
	}
	let path_to_rom = &args[1];
	println!("Loading ROM: {}", path_to_rom);
    let rom: Vec<u8> = utils::read_file(path_to_rom).unwrap();
    println!("Rom loaded: {} bytes", rom.len());

    // init the emulator
    let mut chip8 = Chip8::new();
    chip8.load_rom(rom);

	let mut window: PistonWindow = WindowSettings::new("Rusty8 - chip8 emulator", [640, 480])
		.exit_on_esc(true).build().unwrap();

	while let Some(e) = window.next() {
		// TODO input
		chip8.tick();

		// draw
		if chip8.draw_requested() {
			window.draw_2d(&e, |c, g| {
				clear([0.0; 4], g);
				for x in 0..64 {
		            for y in 0..32 {
		                let draw = chip8.vram[y*64 + x];
		                if draw == 0 {
		                    continue;
		                }
		                let xx: f64 = SCALE * (x as f64);
		                let yy: f64 = SCALE * (y as f64);

						rectangle([1.0, 1.0, 1.0, 1.0], 
						      	[xx, yy, SCALE, SCALE],
						      	c.transform, g);
		            }
		        }
			});
		}

        thread::sleep_ms(1);
	}
    
}