use opengl_graphics::{GlGraphics, TextureSettings};
use opengl_graphics::glyph_cache::GlyphCache;
use graphics::{Transformed, DrawState, Context};
use graphics::text::Text;
use super::DebugInfoController;

const FONT_PATH: &'static str = "assets/OpenSans-Light.ttf";
const FONT_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const FONT_SIZE: u32 = 16;
const TEXT_VERTICAL_SPACING: f64 = 12.0;
const TEXT_HORIZONTAL_POSITION: f64 = 2.0;

/// Provides functionality for rendering visual information
/// pertaining to the status of the program and its performance.
pub struct DebugInfoView {
    /// The cached glyphs rendered using a truetype font.
    glyphs: GlyphCache<'static>,
}

impl DebugInfoView {
    /// Creates and returns a new instance of the DebugInfoView struct
    /// with a pre-populated glyph cache.
    pub fn new() -> DebugInfoView {
        DebugInfoView {
            glyphs: GlyphCache::new(FONT_PATH, TextureSettings::new()).expect(
                "Unable to build glyph cache",
            ),
        }
    }

    /// Draws the information provided by the DebugInfoController to the screen.
    pub fn draw(&mut self, controller: &DebugInfoController, c: &Context, g: &mut GlGraphics) {
        if !controller.should_draw() {
            return;
        }

        let current_fps = controller.current_fps();
        let window_size = controller.window_size();
        let mouse_rel_pos = controller.mouse_relative_pos();
        let mouse_win_pos = controller.mouse_window_pos();
        let mouse_scroll = controller.mouse_scroll();

        self.write_at(
            format!("W/H: {:?}", window_size),
            (TEXT_HORIZONTAL_POSITION, TEXT_VERTICAL_SPACING),
            c,
            g,
        );

        self.write_at(
            format!("Mouse relative: {:?}", mouse_rel_pos),
            (TEXT_HORIZONTAL_POSITION, TEXT_VERTICAL_SPACING * 2.0),
            c,
            g,
        );

        self.write_at(
            format!("Mouse pos: {:?}", mouse_win_pos),
            (TEXT_HORIZONTAL_POSITION, TEXT_VERTICAL_SPACING * 3.0),
            c,
            g,
        );

        self.write_at(
            format!("Mouse scroll: {:?}", mouse_scroll),
            (TEXT_HORIZONTAL_POSITION, TEXT_VERTICAL_SPACING * 4.0),
            c,
            g,
        );

        self.write_at(
            format!("FPS: {:?}", current_fps),
            (TEXT_HORIZONTAL_POSITION, TEXT_VERTICAL_SPACING * 5.0),
            c,
            g,
        );
    }

    /// Helper function for rendering text to an area on the screen.
    fn write_at(&mut self, text: String, pos: (f64, f64), c: &Context, g: &mut GlGraphics) {
        Text::new_color(FONT_COLOR, FONT_SIZE).draw(
            text.as_str(),
            &mut self.glyphs,
            &DrawState::default(),
            c.transform.trans(pos.0, pos.1),
            g,
        );
    }
}
