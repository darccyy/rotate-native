use std::process::exit;

use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::{DrawMode, DrawParam};
use ggez::input::keyboard::{KeyCode, KeyInput, KeyMods};
use ggez::mint::Point2;
use ggez::Context;
use ggez::GameResult;

use crate::color;
use crate::debug::draw_debug_text;

/// Arm colors to render
/// Each color = one arm
const COLORS: &[Color] = &[
    color!(RED),
    color!(GREEN),
    color!(BLUE),
    color!(CYAN),
    color!(YELLOW),
    color!(MAGENTA),
];

/// Width multiplier for arms
const WIDTH_MULTIPLY: f32 = 2.0;
/// Width minimum for arms
const WIDTH_MINIMUM: f32 = 4.0;
/// Length multiplier for arms
const LENGTH_MULTIPLY: f32 = 20.0;
/// Length multiplier for arms
const LENGTH_MINIMUM: f32 = 10.0;
/// Speed of arm rotation (exponential shape)
const SPEED_EXPONENT: f32 = 1.3;
/// Speed of arm rotation (multiplier)
const SPEED_MULTIPLY: f32 = 1.0;

/// Main app
#[derive(Default)]
pub struct App {
    frame_count: u32,
    show_debug: bool,
}

impl EventHandler for App {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.frame_count += 1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, color!(BLACK));

        // Get canvas size
        let (width, height) = ctx.gfx.drawable_size();

        // Center of canvas
        let mut point = Point2 {
            x: width / 2.0,
            y: height / 2.0,
        };

        // Calculate base rotation speed from frame
        let rotation_base = self.frame_count as f32 / 100.0;

        // Render each arm
        for (i, color) in COLORS.into_iter().enumerate() {
            // Arm count from end, as float
            let alpha = (i + 1) as f32;
            // Arm count from center, as float
            let omega = (COLORS.len() - i) as f32;

            // Arm properties
            let width = omega * WIDTH_MULTIPLY + WIDTH_MINIMUM;
            let length = omega * LENGTH_MULTIPLY + LENGTH_MINIMUM;
            let rotation = rotation_base * alpha.powf(SPEED_EXPONENT) * SPEED_MULTIPLY;

            // Calculate point of end of arm
            let next_point = Point2 {
                x: point.x + length * rotation.cos(),
                y: point.y + length * rotation.sin(),
            };

            // Draw arm line
            let mesh = graphics::Mesh::new_line(ctx, &[point, next_point], width, *color)?;
            canvas.draw(&mesh, DrawParam::default());

            /// Inline macro to draw circle at a point
            macro_rules! draw_circle {
                ($point: expr) => {{
                    let mesh = graphics::Mesh::new_circle(
                        ctx,
                        DrawMode::fill(),
                        $point,
                        width / 2.0,
                        0.1,
                        *color,
                    )?;
                    canvas.draw(&mesh, DrawParam::default());
                }};
            }

            // Draw circle (line cap) at each end of arm
            draw_circle!(point);
            draw_circle!(next_point);

            // Update position for next arm
            point = next_point;
        }

        // Display debug information
        if self.show_debug {
            draw_debug_text(
                &mut canvas,
                ctx,
                [
                    format!("Frames rendered: {}", self.frame_count),
                    format!("Arm count: {}", COLORS.len()),
                    format!("Arm width: {} * x + {}", WIDTH_MULTIPLY, WIDTH_MINIMUM),
                    format!("Arm length: {} * x + {}", LENGTH_MULTIPLY, LENGTH_MINIMUM),
                    format!("Rotation speed: {} * x ^ {}", SPEED_MULTIPLY, SPEED_EXPONENT),
                ],
            )?;
        }

        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        use KeyCode::*;

        match (input.mods, input.keycode) {
            // Toggle debug mode
            (KeyMods::NONE, Some(F3)) => self.show_debug ^= true,

            // Exit program
            (_, Some(Escape | Space)) => exit(0),

            _ => (),
        }

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), ggez::GameError> {
        exit(0)
    }
}
