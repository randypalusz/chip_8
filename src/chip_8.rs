use super::cpu::CPU;
use super::display::Display;

use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::Context;
use ggez::GameError;
use ggez::GameResult;
// use ggez_egui::EguiBackend;

const DESIRED_FPS: u32 = 60;

#[allow(non_camel_case_types, dead_code)]
pub struct CHIP_8 {
    cpu: CPU,
    display: Display,
    // TODO: uncomment when ggez_egui is updated
    // egui_backend: EguiBackend,
}

impl CHIP_8 {
    pub fn new(window_height: f32) -> CHIP_8 {
        CHIP_8 {
            cpu: CPU::new(),
            display: Display::new(window_height),
            // TODO: uncomment when ggez_egui is updated
            // egui_backend: EguiBackend::default(),
        }
    }
}

impl ggez::event::EventHandler<GameError> for CHIP_8 {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // TODO: uncomment when ggez_egui is updated
        // let egui_ctx = self.egui_backend.ctx();
        // egui::Window::new("egui window").show(&egui_ctx, |ui| {
        //     ui.set_min_size((500.0, 500.0).into());
        //     egui_ctx.set_pixels_per_point(10.0);
        //     ui.label("label");
        //     if ui.button("print frametime").clicked() {
        //         println!("frametime:");
        //     }
        //     if ui.button("quit").clicked() {
        //         ggez::event::request_quit(ctx);
        //     }
        // });
        // self.egui_backend.update(ctx);

        while ctx.time.check_update_time(DESIRED_FPS) {
            let _seconds = 1.0 / (DESIRED_FPS as f32);
            // println!("frametime: {}", _seconds);

            // execute next cpu cycle here
            self.cpu.execute();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        //TODO: call self.display.set_pixels(<cpu.ram_slice where display data is contained>)
        let mut canvas =
            ggez::graphics::Canvas::from_frame(ctx, ggez::graphics::Color::new(0.2, 0.2, 0.2, 1.0));
        let mut result = self.display.render(ctx, &mut canvas);

        // TODO: uncomment when ggez_egui is updated to support ggez 8.0.0
        // canvas.draw(&self.egui_backend, ggez::graphics::DrawParam::default());

        // finish the canvas assuming the chip_8 display could be rendered
        // otherwise, propogate the error
        result = match result {
            Ok(()) => canvas.finish(ctx),
            Err(e) => Err(e),
        };
        ggez::timer::yield_now();
        result
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult {
        match input.keycode {
            Some(KeyCode::A) => {
                println!("Pressed 'A'");
            }
            _ => (), // do nothing on any other press
        };
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyInput) -> GameResult {
        match keycode.keycode {
            Some(KeyCode::A) => {
                println!("Released 'A'");
            }
            _ => (), // do nothing on any other press
        };
        Ok(())
    }
}
