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

pub mod process;
pub mod server;
pub mod chip8;
pub mod utils;
pub mod frontend;

#[cfg(test)]
mod tests {

	use rusty8::utils::Stack;

	#[test]
    fn stack_basics() {
    	let mut stack = Stack::new();

    	stack.push(7);
    	assert_eq!(stack.pop(), 7);

    	stack.push(13);
    	stack.push(9);
    	stack.push(7);
    	stack.push(3);
    	stack.push(8);
    	assert_eq!(stack.pop(), 8);
    	assert_eq!(stack.pop(), 3);
    	assert_eq!(stack.pop(), 7);
    	assert_eq!(stack.pop(), 9);
    	assert_eq!(stack.pop(), 13);
    }

    #[test]
    #[should_panic]
    fn stack_overflow() {
    	let mut stack = Stack::new();
    	stack.push(7);
    	stack.pop();
    	stack.pop();
    }

    #[test]
    #[should_panic]
    fn stack_underflow() {
    	let mut stack = Stack::new();
    	for i in 0..17 {
    		stack.push(i);
    	}
    }

    #[test]
    fn stack_test_limits() {
    	let mut stack = Stack::new();
    	for i in 0..15 {
    		stack.push(i);
    	}

    	for i in 0..15 {
    		stack.pop();
    	}
    }

}