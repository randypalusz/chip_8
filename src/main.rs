mod chip_8;
mod cpu;
mod display;
mod memory;

use ggez::{conf, Context, ContextBuilder};

const DEFAULT_WINDOW_HEIGHT: f32 = 500.0;

fn main() {
    // only parsing the window height for now
    let win_height: f32 = match std::env::args().nth(1) {
        None => DEFAULT_WINDOW_HEIGHT,
        Some(num) => num.parse::<f32>().unwrap(),
    };

    let instance = chip_8::CHIP_8::new(win_height);

    let c = conf::Conf::new();
    let (ctx, event_loop): (Context, ggez::winit::event_loop::EventLoop<()>) =
        ContextBuilder::new("CHIP_8 EMULATOR", "randypalusz")
            .default_conf(c)
            .window_mode(
                conf::WindowMode::default()
                    .resizable(false)
                    .dimensions(win_height * 3.0, win_height),
            )
            .window_setup(conf::WindowSetup::default().title("CHIP_8").vsync(true))
            .build()
            .unwrap();

    ggez::event::run(ctx, event_loop, instance);
}
