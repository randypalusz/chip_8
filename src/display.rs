use glutin_window::GlutinWindow as Window;
use graphics::types::Color;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::RenderArgs;
use piston::window::WindowSettings;

const NUM_PIXELS_Y: usize = 32;
const NUM_PIXELS_X: usize = 64;
const NUM_PIXELS: usize = NUM_PIXELS_X * NUM_PIXELS_Y;

type PixelsArray = [Pixel; NUM_PIXELS];

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    start_x: f64,
    start_y: f64,
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
            .resizable(false)
            .build()
            .unwrap();
        let renderer: GlGraphics = opengl_graphics::GlGraphics::new(opengl);

        let mut pixels: PixelsArray = Display::init_pixels(&pixel_size, &width, &height);

        Display::pixel_test(&mut pixels);

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

    //TODO: remove this, just being used to test display for now
    fn pixel_test(pixels: &mut PixelsArray) {
        let mut first_state_toggle: bool = false;
        let mut state_toggle: bool = first_state_toggle;

        for (idx, mut pixel) in pixels.iter_mut().enumerate() {
            state_toggle = !state_toggle;
            if idx % NUM_PIXELS_X == 0 {
                state_toggle = first_state_toggle;
                first_state_toggle = !first_state_toggle;
            }
            pixel.state = state_toggle;
        }
    }

    fn init_pixels(pixel_size: &f64, window_width: &f64, window_height: &f64) -> PixelsArray {
        let mut pixels: PixelsArray = [Pixel {
            start_x: 0.0,
            start_y: 0.0,
            state: false,
        }; NUM_PIXELS];

        let mut current_x: f64 = 0.0;
        let mut current_y: f64 = 0.0;

        for mut pixel in pixels.iter_mut() {
            if current_x >= *window_width {
                current_x = 0.0;
                current_y += window_height / NUM_PIXELS_Y as f64;
            }

            pixel.start_x = current_x;
            pixel.start_y = current_y;

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
                let transform = c.transform.trans(pixel.start_x, pixel.start_y);
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
