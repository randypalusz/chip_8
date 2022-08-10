use ggez::graphics::{self, Canvas, Color};
use mint::Point2;

const NUM_PIXELS_Y: usize = 32;
const NUM_PIXELS_X: usize = 64;
const NUM_PIXELS: usize = NUM_PIXELS_X * NUM_PIXELS_Y;

type PixelsArray = [Pixel; NUM_PIXELS];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PixelState {
    On,
    Off,
}

impl std::ops::Not for PixelState {
    type Output = PixelState;
    fn not(self) -> PixelState {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    start_x: f32,
    start_y: f32,
    state: PixelState, // false is off, true is on
}

pub struct Display {
    _width: f32,  // window width
    _height: f32, // window height
    _pixel_size: f32,
    pixel_template: graphics::Rect,
    pub pixels: [Pixel; NUM_PIXELS], //indexing for pixels (false = off, true = on)
    off_color: graphics::Color,      //Color consts
    on_color: graphics::Color,       //Color consts
}

impl Display {
    pub fn new(window_height: f32) -> Display {
        let height = window_height;
        let width = height * 2.0;
        let pixel_size: f32 = height / (NUM_PIXELS_Y as f32);

        let pixel_template = graphics::Rect::new(0.0, 0.0, pixel_size, pixel_size);

        let mut pixels: PixelsArray = Display::init_pixels(&pixel_size, &width, &height);

        Display::pixel_test(&mut pixels);

        Display {
            _width: width,
            _height: height,
            _pixel_size: pixel_size,
            pixel_template,
            pixels,
            off_color: Color::BLACK,
            on_color: Color::WHITE,
        }
    }

    //TODO: remove this, just being used to test display for now
    fn pixel_test(pixels: &mut PixelsArray) {
        let mut first_state_toggle: PixelState = PixelState::Off;
        let mut state_toggle: PixelState = first_state_toggle;

        for (idx, mut pixel) in pixels.iter_mut().enumerate() {
            state_toggle = !state_toggle;
            if idx % NUM_PIXELS_X == 0 {
                state_toggle = first_state_toggle;
                first_state_toggle = !first_state_toggle;
            }
            pixel.state = state_toggle;
        }
    }

    fn init_pixels(pixel_size: &f32, window_width: &f32, window_height: &f32) -> PixelsArray {
        let mut pixels: PixelsArray = [Pixel {
            start_x: 0.0,
            start_y: 0.0,
            state: PixelState::Off,
        }; NUM_PIXELS];

        let mut current_x: f32 = 0.0;
        let mut current_y: f32 = 0.0;

        for mut pixel in pixels.iter_mut() {
            if current_x >= *window_width {
                current_x = 0.0;
                current_y += window_height / NUM_PIXELS_Y as f32;
            }

            pixel.start_x = current_x;
            pixel.start_y = current_y;

            current_x += pixel_size;
        }
        pixels
    }

    #[allow(dead_code)]
    pub fn set_pixel(&mut self, pixel: usize, value: PixelState) {
        self.pixels[pixel].state = value;
    }

    // TODO: Test that this works
    #[allow(dead_code)]
    pub fn set_pixels(&mut self, pixels: Vec<usize>, values: Vec<PixelState>) {
        for (pixel, value) in pixels.iter().zip(values.iter()) {
            self.pixels[*pixel].state = *value;
        }
    }

    pub fn render(&mut self, ctx: &mut ggez::Context, canvas: &mut Canvas) -> ggez::GameResult<()> {
        for pixel in self.pixels {
            let draw_color = if pixel.state == PixelState::On {
                self.on_color
            } else {
                self.off_color
            };
            let transform: Point2<f32> = Point2 {
                x: pixel.start_x,
                y: pixel.start_y,
            };

            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest(transform)
                    .color(draw_color)
                    .scale(self.pixel_template.size()),
            );
        }
        Ok(())
    }
}
