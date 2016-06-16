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

use std::time::Duration;
use std::thread;

pub struct Process<T: Frontend> {
    /// Frontend, used for rendering & input
    frontend:   T,
    /// emulator
    chip8:  Chip8,
    /// is our process still running?
    running: bool,
    /// Sleep time
    clock: Duration,
}

impl<T: Frontend> Process<T> {

    pub fn new(frontend: T) -> Process<T> {
        let process = Process {
            frontend:   frontend,
            chip8:      Chip8::new(),
            running:    false,
            clock:      Duration::from_millis(10),
        };

        return process;
    }

    pub fn run(&mut self, rom: Vec<u8>) {
        // load rom
        self.chip8.load_rom(rom);
        self.running = true;

        // start emulating
        while self.is_running() {
            self.frontend.do_input(&mut self.chip8);
            self.chip8.tick();
            
            // draw
            if self.chip8.draw_requested() {
                self.frontend.render(&self.chip8);
            }   

            // emulate cpu clock
            thread::sleep(self.clock);
        }
    }

    pub fn is_running(&self) -> bool {
        return self.running;
    }

}