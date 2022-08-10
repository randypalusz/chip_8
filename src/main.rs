mod cpu;
mod display;
mod memory;

use cpu::CPU;
use display::Display;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::ButtonEvent;

fn main() {
    // TODO: wrap this stuff in a CHIP_8 module
    let _cpu = CPU::new();
    let mut display = Display::new();
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut display.window) {
        if let Some(args) = e.render_args() {
            // pass the ram into the render function to display the pixels as they exist in memory
            display.render(&args);
        }

        if let Some(args) = e.button_args() {
            let _button_state = args.state;
            let _button = args.button;
            // TODO: call device.set_key(button, button_state) to set ram based on the key input
            // println!("{:?}: {:?}", button_state, button);
        }

        if let Some(_args) = e.update_args() {
            // TODO: call device.update(args.dt) or something to update timers based on the program's run speed
        }
    }
}
