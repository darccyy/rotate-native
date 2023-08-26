extern crate good_web_game as ggez;

mod app;
mod debug;

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

fn main() -> GameResult<()> {
    // Start app
    ggez::start(ggez::conf::Conf::default(), |mut context, quad_ctx| {
        Box::new(App::new(&mut context, quad_ctx).unwrap())
    })
}
