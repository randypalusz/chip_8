use glutin_window::GlutinWindow as Window;
use graphics::types::Color;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const NUM_PIXELS_Y: usize = 32;
const NUM_PIXELS_X: usize = 64;
const NUM_PIXELS: usize = NUM_PIXELS_X * NUM_PIXELS_Y;

type PixelsArray = [Pixel; NUM_PIXELS];

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    center_x: f64,
    center_y: f64,
    state: bool, // false is off, true is on
}

pub struct Display {
    width: f64,  // window width
    height: f64, // window height
    pixel_size: f64,
    pub pixels: [Pixel; NUM_PIXELS], //indexing for pixels (false = off, true = on)
    off_color: Color,                //Color consts
    on_color: Color,                 //Color consts
    pub window: Window,
    renderer: GlGraphics,
}

impl Display {
    pub fn new() -> Display {
        let height = 500.0;
        let width = height * 2.0;
        let pixel_size: f64 = height / (NUM_PIXELS_Y as f64);

        let opengl = OpenGL::V3_2;
        let window: Window = WindowSettings::new("CHIP-8", [width, height])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
        let renderer: GlGraphics = opengl_graphics::GlGraphics::new(opengl);

        let pixels: PixelsArray = Display::init_pixels(&pixel_size, &width, &height);

        Display {
            width,
            height,
            pixel_size,
            pixels,
            off_color: [0.0, 0.0, 0.0, 1.0],
            on_color: [1.0, 1.0, 1.0, 1.0],
            window,
            renderer,
        }
    }

    // TODO: Seems like the pixel initializtion is off - not starting at exactly the right spot
    fn init_pixels(pixel_size: &f64, window_width: &f64, window_height: &f64) -> PixelsArray {
        let mut pixels: PixelsArray = [Pixel {
            center_x: 0.0,
            center_y: 0.0,
            state: false,
        }; NUM_PIXELS];

        let mut current_x: f64 = pixel_size / 2.0;
        let mut current_y: f64 = pixel_size / 2.0;

        let mut state_toggle: bool = false;

        for mut pixel in pixels.iter_mut() {
            state_toggle = !state_toggle;
            if current_x >= *window_width {
                println!("here: current_x: {current_x}, current_y: {current_y}");
                current_x = pixel_size / 2.0;
                current_y += window_height / NUM_PIXELS_Y as f64;
            }

            pixel.center_x = current_x;
            pixel.center_y = current_y;
            pixel.state = state_toggle;

            current_x += pixel_size;
        }
        // println!("{:?}", pixels);
        pixels
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, self.pixel_size);
        self.renderer.draw(args.viewport(), |c, gl| {
            clear(self.off_color, gl);

            for pixel in self.pixels {
                let transform = c.transform.trans(pixel.center_x, pixel.center_y);
                let draw_color = if pixel.state {
                    self.on_color
                } else {
                    self.off_color
                };
                rectangle(draw_color, square, transform, gl);
            }
        });
    }
}
