mod chip_8;
mod display;
mod memory;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub fn render(rotation: &f64, args: &RenderArgs, renderer: &mut GlGraphics) {
    use graphics::*;

    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    let square = rectangle::square(0.0, 0.0, 50.0);
    let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

    renderer.draw(args.viewport(), |c, gl| {
        // Clear the screen.
        clear(GREEN, gl);

        let transform = c
            .transform
            .trans(x, y)
            .rot_rad(*rotation)
            .trans(-25.0, -25.0);

        // Draw a box rotating around the middle of the screen.
        rectangle(RED, square, transform, gl);
    });
}
pub fn update(rotation: f64, args: &UpdateArgs) {}

fn main() {
    chip_8::init();
    let mut display = display::Display::new();
    let mut events = Events::new(EventSettings::new());
    //println!("{:?}", display.pixels);
    while let Some(e) = events.next(&mut display.window) {
        if let Some(args) = e.render_args() {
            // render(&rotation, &args, &mut renderer)
            display.render(&args);
        }

        if let Some(args) = e.update_args() {}
    }
}
