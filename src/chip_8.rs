use super::cpu::CPU;
use super::display::Display;

use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::Context;
use ggez::GameError;
use ggez::GameResult;

const DESIRED_FPS: u32 = 60;

#[allow(non_camel_case_types, dead_code)]
pub struct CHIP_8 {
    cpu: CPU,
    display: Display,
}

impl CHIP_8 {
    pub fn new(window_height: f32) -> CHIP_8 {
        CHIP_8 {
            cpu: CPU::new(),
            display: Display::new(window_height),
        }
    }
}

impl ggez::event::EventHandler<GameError> for CHIP_8 {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ctx.time.check_update_time(DESIRED_FPS) {
            let _seconds = 1.0 / (DESIRED_FPS as f32);
            // println!("frametime: {}", seconds);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        //TODO: call self.display.set_pixels(<cpu.ram_slice where display data is contained>)
        let result = self.display.render(ctx);
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
