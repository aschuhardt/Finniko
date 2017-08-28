use opengl_graphics::GlGraphics;
use graphics::{Transformed, DrawState, Context};
use text_renderer::{FontSize, TextRenderer};
use super::DebugInfoController;

const TEXT_VERTICAL_SPACING: f64 = 12.0;
const TEXT_HORIZONTAL_POSITION: f64 = 2.0;

/// Provides functionality for rendering visual information
/// pertaining to the status of the program and its performance.
pub struct DebugInfoView {
    text_renderer: &'static TextRenderer,
}

impl DebugInfoView {
    /// Creates and returns a new instance of the DebugInfoView struct
    /// with a pre-populated glyph cache.
    pub fn new(txt: &'static TextRenderer) -> DebugInfoView {
        DebugInfoView { text_renderer: txt }
    }

    /// Draws the information provided by the DebugInfoController to the screen.
    pub fn draw(&mut self, controller: &DebugInfoController, c: &Context, g: &mut GlGraphics) {
        if !controller.should_draw() {
            return;
        }

        let current_fps = controller.framerate();
        let window_size = controller.window_size();
        let mouse_rel_pos = controller.mouse_relative_pos();
        let mouse_win_pos = controller.mouse_window_pos();
        let mouse_scroll = controller.mouse_scroll();

        self.write_at(
            format!("W/H: {:?}", window_size),
            [TEXT_HORIZONTAL_POSITION, TEXT_VERTICAL_SPACING * 0.0],
            c,
            g,
        );

        self.write_at(
            format!("Mouse relative: {:?}", mouse_rel_pos),
            [TEXT_HORIZONTAL_POSITION, TEXT_VERTICAL_SPACING * 1.0],
            c,
            g,
        );

        self.write_at(
            format!("Mouse pos: {:?}", mouse_win_pos),
            [TEXT_HORIZONTAL_POSITION, TEXT_VERTICAL_SPACING * 2.0],
            c,
            g,
        );

        self.write_at(
            format!("Mouse scroll: {:?}", mouse_scroll),
            [TEXT_HORIZONTAL_POSITION, TEXT_VERTICAL_SPACING * 3.0],
            c,
            g,
        );

        self.write_at(
            format!("FPS: {:?}", current_fps),
            [TEXT_HORIZONTAL_POSITION, TEXT_VERTICAL_SPACING * 4.0],
            c,
            g,
        );
    }

    /// Helper function for rendering text to an area on the screen.
    fn write_at(&mut self, text: String, pos: [f64; 2], c: &Context, g: &mut GlGraphics) {
        self.text_renderer.draw_at(
            pos,
            text,
            c.transform,
            FontSize::Size18,
            [1.0, 0.0, 0.0, 1.0],
            g,
        );
    }
}
