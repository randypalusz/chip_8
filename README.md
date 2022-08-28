# CHIP-8 Emulator

## Goals
* Create an emulator for the CHIP-8 platform using this as a guide: https://tobiasvl.github.io/blog/write-a-chip-8-emulator/, avoid code examples unless absolutely necessary
* Written in Rust
* Keep graphics simple, using the Piston engine as a wrapper for opengl to draw to the screen (https://github.com/PistonDevelopers/piston)
* Target MacOSX/Linux


## Build
* Install rustup, cargo as the package manager, then simply execute:

      cargo build

## Running
* Run in release mode via the following:

      cargo run <optional_window_width_in_pixels> --release