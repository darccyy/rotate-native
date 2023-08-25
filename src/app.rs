use ggez::event::EventHandler;
use ggez::graphics;
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::miniquad::GraphicsContext;
use ggez::Context;
use ggez::GameResult;

use crate::debug::draw_debug_text;
use crate::color;

#[derive(Default)]
pub struct App {
    frame_count: u32,
    show_debug: bool,
}

impl App {
    pub fn new(_ctx: &mut Context, _quad_ctx: &mut GraphicsContext) -> GameResult<Self> {
        Ok(Self::default())
    }
}

impl EventHandler for App {
    fn update(&mut self, _ctx: &mut Context, _quad_ctx: &mut GraphicsContext) -> GameResult {
        self.frame_count += 1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut GraphicsContext) -> GameResult {
        graphics::clear(ctx, quad_ctx, color!(BLACK));

        if self.show_debug {
            draw_debug_text(
                ctx,
                quad_ctx,
                [format!("Total frames: {}", self.frame_count)],
            )?;
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut GraphicsContext,
        keycode: KeyCode,
        keymod: KeyMods,
        _repeat: bool,
    ) {
        use KeyCode::*;

        match (keymod, keycode) {
            (KeyMods::NONE, F3) => self.show_debug ^= true,
            _ => (),
        }
    }
}
