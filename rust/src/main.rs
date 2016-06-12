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

mod r8;

use r8::server::{Chip8Server};

const DEFAULT_PORT: u16 = 7890;

fn main() {
    let server = Chip8Server::new("Rusty8 emulation server", DEFAULT_PORT);
    server.start();
}