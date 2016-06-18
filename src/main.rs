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
use std::thread;

use piston_window::*;

use rusty8::utils;
use rusty8::chip8::Chip8;

const SCALE: f64 = 8.0;

fn main() {
    let rom: Vec<u8> = load_rom();

    // init the emulator
    let mut chip8 = Chip8::new();
    chip8.load_rom(rom);

    // Init piston 
    let mut window: PistonWindow = WindowSettings::new("Rusty8 - chip8 emulator", 
        [640, 480]).exit_on_esc(true).build().unwrap();
    let mut keys: [bool; 16] = [false; 16];

    // Game loop
    while let Some(e) = window.next() { 
        // fetch input events
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::D0 => keys[0] = true,
                Key::D1 => keys[1] = true,
                Key::D2 => keys[2] = true,
                Key::D3 => keys[3] = true,
                Key::D4 => keys[4] = true,
                Key::D5 => keys[5] = true,
                Key::D6 => keys[6] = true,
                Key::D7 => keys[7] = true,
                Key::D8 => keys[8] = true,
                Key::D9 => keys[9] = true,
                Key::A => keys[10] = true,
                Key::B => keys[11] = true,
                Key::C => keys[12] = true,
                Key::D => keys[13] = true,
                Key::E => keys[14] = true,
                Key::F => keys[15] = true,
                _ => {},
            }
        };
        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::D0 => keys[0] = false,
                Key::D1 => keys[1] = false,
                Key::D2 => keys[2] = false,
                Key::D3 => keys[3] = false,
                Key::D4 => keys[4] = false,
                Key::D5 => keys[5] = false,
                Key::D6 => keys[6] = false,
                Key::D7 => keys[7] = false,
                Key::D8 => keys[8] = false,
                Key::D9 => keys[9] = false,
                Key::A => keys[10] = false,
                Key::B => keys[11] = false,
                Key::C => keys[12] = false,
                Key::D => keys[13] = false,
                Key::E => keys[14] = false,
                Key::F => keys[15] = false,
                _ => {},
            }
        };

        // execute next cycle
        chip8.set_keys(&keys);
        chip8.tick();

        // Draw if flag is set
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
                        rectangle([1.0, 1.0, 1.0, 1.0], [xx, yy, SCALE, SCALE], c.transform, g);
                    }
                }
            });
        }

        // sleep a bit
        thread::sleep_ms(1);
    }
}


// Loads rom or panics
fn load_rom() -> Vec<u8> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 { 
        panic!("No rom specified");
    }

    let path_to_rom = &args[1];
    println!("Loading ROM: {}", path_to_rom);
    match utils::read_file(path_to_rom) {
        Ok(rom) => {
            println!("Rom loaded: {} bytes", rom.len());
            return rom;
        },
        Err(err) => panic!("Rom not found"),
    }
}