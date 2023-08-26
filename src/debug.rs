use ggez::{
    cgmath::Point2,
    graphics::{self, Color, DrawMode, DrawParam, TextFragment},
    miniquad, Context, GameResult,
};

use crate::color;

/// Margin for bounding rectangle
const MARGIN: f32 = 20.0;
/// Padding for text inside rectangle
const PADDING: f32 = 8.0;
/// Text line height
const LINE_HEIGHT: f32 = 5.0;
/// Text font size
const FONT_SIZE: f32 = 18.0;
/// Text color
const COLOR_TEXT: Color = color!(WHITE);
/// Bounding rectangle color
const COLOR_BG: Color = color!(128, 0, 0, 128);

/// Render debug information on canvas
pub fn draw_debug_text<const N: usize>(
    ctx: &mut Context,
    quad_ctx: &mut miniquad::GraphicsContext,
    lines: [impl Into<TextFragment>; N],
) -> GameResult {
    // Skip if no debug lines
    if lines.is_empty() {
        return Ok(());
    }

    // Get canvas size
    let (width, height) = quad_ctx.screen_size();

    // Bounding rectangle properties
    let rect_height =
        PADDING * 2.0 + FONT_SIZE * lines.len() as f32 + LINE_HEIGHT * (lines.len() as f32 - 1.0);
    let rect_width = width - MARGIN * 2.0;
    let rect_x = MARGIN;
    let rect_y = height - MARGIN - rect_height;

    // Draw bounding rectangle
    let rect = graphics::Rect::new(rect_x, rect_y, rect_width, rect_height);
    let mesh = graphics::Mesh::new_rectangle(ctx, quad_ctx, DrawMode::fill(), rect, COLOR_BG)?;
    graphics::draw(ctx, quad_ctx, &mesh, DrawParam::default())?;

    // Draw each line
    for (i, line) in lines.into_iter().enumerate() {
        // Get position from rectangle properties
        let position = Point2::new(
            rect_x + PADDING,
            rect_y + PADDING + (FONT_SIZE + LINE_HEIGHT) * i as f32,
        );
        // Draw text
        let text = graphics::Text::new((line, graphics::Font::default(), FONT_SIZE));
        graphics::draw(ctx, quad_ctx, &text, (position, 0.0, COLOR_TEXT))?;
    }
    Ok(())
}
