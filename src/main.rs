mod chip_8;
mod cpu;
mod display;
mod memory;

use ggez::{conf, Context, ContextBuilder};

const WINDOW_HEIGHT: f32 = 500.0;

// TODO: replace piston with ggez
fn main() {
    let instance = chip_8::CHIP_8::new(WINDOW_HEIGHT);

    let c = conf::Conf::new();
    let (ctx, event_loop): (Context, ggez::winit::event_loop::EventLoop<()>) =
        ContextBuilder::new("CHIP_8 EMULATOR", "randypalusz")
            .default_conf(c)
            .window_mode(
                conf::WindowMode::default()
                    .resizable(false)
                    .dimensions(WINDOW_HEIGHT * 2.0, WINDOW_HEIGHT),
            )
            .window_setup(conf::WindowSetup::default().title("CHIP_8").vsync(true))
            .build()
            .unwrap();

    ggez::event::run(ctx, event_loop, instance);
}
