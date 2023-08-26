use ggez::{
    graphics::{self, Canvas, Color, DrawMode, DrawParam, TextFragment},
    mint::Point2,
    Context, GameResult,
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
    canvas: &mut Canvas,
    ctx: &mut Context,
    lines: [impl Into<TextFragment>; N],
) -> GameResult {
    // Skip if no debug lines
    if lines.is_empty() {
        return Ok(());
    }

    // Get canvas size
    let (width, height) = ctx.gfx.drawable_size();

    // Bounding rectangle properties
    let rect_height =
        PADDING * 2.0 + FONT_SIZE * lines.len() as f32 + LINE_HEIGHT * (lines.len() as f32 - 1.0);
    let rect_width = width - MARGIN * 2.0;
    let rect_x = MARGIN;
    let rect_y = height - MARGIN - rect_height;

    // Draw bounding rectangle
    let rect = graphics::Rect::new(rect_x, rect_y, rect_width, rect_height);
    let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, COLOR_BG)?;
    canvas.draw(&mesh, DrawParam::default());

    // Draw each line
    for (i, line) in lines.into_iter().enumerate() {
        // Get position from rectangle properties
        let position = Point2 {
            x: rect_x + PADDING,
            y: rect_y + PADDING + (FONT_SIZE + LINE_HEIGHT) * i as f32,
        };

        // Draw text
        let mut text = graphics::Text::new(line);
        text.set_scale(FONT_SIZE);
        let param = DrawParam::default().dest(position).color(COLOR_TEXT);
        canvas.draw(&text, param);
    }
    Ok(())
}
