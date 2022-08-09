mod chip_8;
mod display;
mod memory;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};

fn main() {
    chip_8::init();
    let mut display = display::Display::new();
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut display.window) {
        if let Some(args) = e.render_args() {
            display.render(&args);
        }

        if let Some(args) = e.update_args() {}
    }
}
