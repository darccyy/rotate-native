mod app;
mod debug;

use ggez::conf::{FullscreenType, WindowMode};
use ggez::event;
use ggez::ContextBuilder;
use ggez::GameResult;

use app::App;

/// Returns `ggez::graphics::Color` value, as const
#[macro_export]
macro_rules! color {
    ($name:ident) => {
        ::ggez::graphics::Color::$name
    };
    ($r:expr, $g:expr, $b:expr) => {
        ::ggez::graphics::Color::new(
            $r as u8 as f32 / 255.0,
            $g as u8 as f32 / 255.0,
            $b as u8 as f32 / 255.0,
            255.0,
        )
    };
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        ::ggez::graphics::Color::new(
            $r as u8 as f32 / 255.0,
            $g as u8 as f32 / 255.0,
            $b as u8 as f32 / 255.0,
            $a as u8 as f32 / 255.0,
        )
    };
}

fn main() -> GameResult {
    // Create app context
    let (mut ctx, event_loop) = ContextBuilder::new("rotating-arms", "darcy").build()?;

    // Change window properties
    ctx.gfx.set_window_title("Rotating Arms");
    ctx.gfx.set_mode(WindowMode {
        width: 1680.0,
        height: 1050.0,
        fullscreen_type: FullscreenType::True,
        ..Default::default()
    })?;

    // Create app state
    let app = App::default();

    // Run game loop
    event::run(ctx, event_loop, app);
}
