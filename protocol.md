# rusty8 backend <-> frontend communication protocol

## 1. Introduction
The backend listens on a tcp port and accepts new requests made by the frontend.
If a new client (frontend) connects to the server (backend), a new thread will be created
and all subsequent actions + the emulation of the chip-8 program will be done in that thread.

This makes it possible to connect multiple fontends to the backend simultaneously.

## 2. Connections
