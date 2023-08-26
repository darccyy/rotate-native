use std::f32::consts::PI;

use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::{DrawMode, DrawParam};
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::miniquad::GraphicsContext;
use ggez::Context;
use ggez::GameResult;
use good_web_game::mint::Point2;

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
const WIDTH_MULTIPLY: f32 = 3.0;
/// Width minimum for arms
const WIDTH_MINIMUM: f32 = 2.0;
/// Length multiplier for arms
const LENGTH_MULTIPLY: f32 = 20.0;
/// Length multiplier for arms
const LENGTH_MINIMUM: f32 = 10.0;
/// Speed of arm rotation (exponential shape)
const SPEED_EXPONENT: f32 = 1.3;
/// Speed of arm rotation (multiplier)
const SPEED_MULTIPLY: f32 = 0.01;

/// Main app
#[derive(Default)]
pub struct App {
    frame_count: u32,
    show_debug: bool,
}

impl App {
    /// Create new app state (default)
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

        // Get canvas size
        let (width, height) = quad_ctx.screen_size();

        // Center of canvas
        let mut point = Point2 {
            x: width / 2.0,
            y: height / 2.0,
        };

        for (i, color) in COLORS.into_iter().enumerate() {
            // Arm count from center, as float
            let alpha = (COLORS.len() - i) as f32;
            // Arm count from end, as float
            let omega = (i + 1) as f32;

            // Arm properties
            let width = alpha * WIDTH_MULTIPLY + WIDTH_MINIMUM;
            let length = alpha * LENGTH_MULTIPLY + LENGTH_MINIMUM;
            let rotation = self.frame_count as f32 * omega.powf(SPEED_EXPONENT) * SPEED_MULTIPLY;

            // Points for arm line
            let points = [Point2 { x: 0.0, y: 0.0 }, Point2 { x: 0.0, y: length }];
            // Use rotation transformation
            let param = DrawParam::new().dest(point).rotation(rotation - PI / 2.0);

            // Draw arm line
            let mesh = graphics::MeshBuilder::new()
                .line(&points, width, *color)?
                .build(ctx, quad_ctx)?;
            graphics::draw(ctx, quad_ctx, &mesh, param)?;

            // Draw circle (line cap) at start of arm
            let mesh = graphics::Mesh::new_circle(
                ctx,
                quad_ctx,
                DrawMode::fill(),
                point,
                width / 2.0,
                0.1,
                *color,
            )?;
            graphics::draw(ctx, quad_ctx, &mesh, DrawParam::default())?;

            // Move point to end of arm
            point.x += length * rotation.cos();
            point.y += length * rotation.sin();

            // Draw circle (line cap) at end of arm
            let mesh = graphics::Mesh::new_circle(
                ctx,
                quad_ctx,
                DrawMode::fill(),
                point,
                width / 2.0,
                0.1,
                *color,
            )?;
            graphics::draw(ctx, quad_ctx, &mesh, DrawParam::default())?;
        }

        // Display debug information
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
        match (keymod, keycode) {
            // Toggle debug mode
            (KeyMods::NONE, KeyCode::F3) => self.show_debug ^= true,

            _ => (),
        }
    }
}
