# rusty8

rusty8 is a chip-8 emulator.
The backend is written in Rust, the frontend in NodeJS + Electron.

## How does it work
The backend emulates the program, whereas the frontend is responsible for rendering and input.
Both parts communicate over TCP. One connection from frontend to backend for
sending user input and one connection from backend to frontend for sending the data
for rendering.

..............      data       .............
.            . --------------> .           .
.  Backend   .      input      . Frontend  .
.            . <-------------- .           .
..............                 .............

## Backend
The emulation of the chip-8 chip & execution of the program is done in the backend, which is written in Rust.
Rust is a great language for this job, since it is (amongst other things) super fast.

### Compiling the backend
You will need to install the Rust programming language and the package manager cargo.
[INSERT LINK HERE]

Compile and auto-run: cd rusty8 && cargo run
Compile for release: cd rusty8 && cargo build --release

For more info about rust & cargo look at their documentation, it's really awesome.

## Frontend
The fronend is written in NodeJS/HTML/CSS. It uses Electron [INSERT LINK HERE], which is a
cross platform desktop application framework. It's developed by Github and used in many projects,
including the Atom editor.

### Building the frontend
You need to install the latest NodeJS version [INSERT LINK] and Node's package manager npm [INSERT LINK].

Then run: cd rusty8/node && npm install && npm start

This fetches all dependencies and runs the app.

