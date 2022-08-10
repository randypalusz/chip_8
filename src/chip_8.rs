use super::cpu::CPU;
use super::display::Display;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::ButtonEvent;

#[allow(non_camel_case_types, dead_code)]
pub struct CHIP_8 {
    cpu: CPU,
    display: Display,
}

impl CHIP_8 {
    pub fn new() -> CHIP_8 {
        CHIP_8 {
            cpu: CPU::new(),
            display: Display::new(),
        }
    }

    //TODO: return Result instead to exit from error state
    pub fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.display.window) {
            if let Some(args) = e.render_args() {
                // pass the ram into the render function to display the pixels as they exist in memory
                self.display.render(&args);
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
}
