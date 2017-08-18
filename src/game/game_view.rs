use std::iter::Iterator;
use graphics::{self, Transformed, DrawState, Context};
use opengl_graphics::{GlGraphics, TextureSettings};
use opengl_graphics::glyph_cache::GlyphCache;
use graphics::text::Text;
use super::message::*;
use super::{MAP_WIDTH, MAP_HEIGHT, GameController, TextureMapper, Drawable};

const FONT_PATH_LIGHT: &'static str = "assets/OpenSans-Light.ttf";
const FONT_PATH_REGULAR: &'static str = "assets/OpenSans-Regular.ttf";
const FONT_PATH_BOLD: &'static str = "assets/OpenSans-Bold.ttf";
const FONT_SIZE: u32 = 16;
const MESSAGE_LINE_HEIGHT: f64 = 18.0;
const MESSAGE_LEFT_PAD: f64 = 10.0;
const MESSAGE_TOP: f64 = 6.0;

enum FontStyle {
    Light,
    Regular,
    Bold,
}

/// Renders information about the game's current state to the screen.
pub struct GameView {
    tm: TextureMapper,
    glyphs_light: GlyphCache<'static>,
    glyphs_regular: GlyphCache<'static>,
    glyphs_bold: GlyphCache<'static>,
}

impl GameView {
    /// Creates and returns a new instance of the GameView struct.
    pub fn new() -> GameView {
        GameView {
            tm: TextureMapper::new(),
            glyphs_light: GlyphCache::new(FONT_PATH_LIGHT, TextureSettings::new()).expect(
                format!(
                    "Unable to build glyph cache: {:?}",
                    FONT_PATH_LIGHT
                ).as_str(),
            ),
            glyphs_regular: GlyphCache::new(FONT_PATH_REGULAR, TextureSettings::new()).expect(
                format!(
                    "Unable to build glyph cache: {:?}",
                    FONT_PATH_REGULAR
                ).as_str(),
            ),
            glyphs_bold: GlyphCache::new(FONT_PATH_BOLD, TextureSettings::new()).expect(
                format!(
                    "Unable to build glyph cache: {:?}",
                    FONT_PATH_BOLD
                ).as_str(),
            ),
        }
    }

    /// Renders visuals onto the window based on the state of the provided controller instance.
    pub fn draw(&mut self, controller: &GameController, c: &Context, g: &mut GlGraphics) {
        let screen_rect = c.viewport
            .unwrap_or_else(|| panic!("Could not get the viewport!"))
            .rect;

        let tile_w = screen_rect[2] as f64 / MAP_WIDTH as f64;
        let tile_h = screen_rect[3] as f64 / MAP_HEIGHT as f64;

        // draw tiles
        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                match controller.tile_sprite_at([x as i32, y as i32]) {
                    Ok(tile_sprite) => {
                        self.tm.draw_at(
                            [
                                x as f64 * tile_w,
                                y as f64 * tile_h,
                                tile_w + 1.0,
                                tile_h + 1.0,
                            ],
                            &tile_sprite,
                            c.transform,
                            g,
                        );
                    }
                    Err(why) => error!("{:?}", why),
                };
            }
        }

        // draw player
        let player = controller.get_player();
        self.tm.draw_at(
            [
                player.position[0] as f64 * tile_w,
                player.position[1] as f64 * tile_h,
                tile_w,
                tile_h,
            ],
            &player.get_sprite_key(),
            c.transform,
            g,
        );

        // draw message
        if controller.should_show_messages() {
            // background
            let screen_w = screen_rect[2] as f64;
            let screen_h = screen_rect[3] as f64;

            // check which side of the screen the player is on,
            // and adjust the position of message box accordingly
            let mut left_adjust = 0.0;
            let player_x = controller.get_player().position[0] as f64;
            if player_x * tile_w < screen_w / 2.0 {
                left_adjust = screen_w - (screen_w / 4.0);
            }

            graphics::rectangle(
                [0.1, 0.1, 0.1, 0.9],
                [left_adjust, 0.0, screen_w / 4.0, screen_h],
                c.transform,
                g,
            );

            // text
            let line_count = (screen_rect[3] as f64 / MESSAGE_LINE_HEIGHT) as usize - 1;
            let messages = controller.get_messages(line_count);
            for i in 0..messages.len() {
                let msg = &messages[i];
                let color = match msg.message_type {
                    MessageType::Normal => MESSAGE_COLOR_NORMAL,
                    MessageType::Danger => MESSAGE_COLOR_DANGER,
                    MessageType::Benefit => MESSAGE_COLOR_BENEFIT,
                    MessageType::Background => MESSAGE_COLOR_BACKGROUND,
                };
                let position = [
                    MESSAGE_LEFT_PAD + left_adjust,
                    (MESSAGE_LINE_HEIGHT * line_count as f64) 
                        - (MESSAGE_LINE_HEIGHT * i as f64) 
                        + MESSAGE_TOP,
                ];
                let text = msg.contents.clone();
                self.write_at(text, FontStyle::Regular, position, color, c, g);
            }
        }
    }

    fn write_at(
        &mut self,
        text: String,
        style: FontStyle,
        pos: [f64; 2],
        color: [f32; 4],
        c: &Context,
        g: &mut GlGraphics,
    ) {
        let glyphs = match style {
            FontStyle::Light => &mut self.glyphs_light,
            FontStyle::Regular => &mut self.glyphs_regular,
            FontStyle::Bold => &mut self.glyphs_bold,
        };
        Text::new_color(color, FONT_SIZE).draw(
            text.as_str(),
            glyphs,
            &DrawState::default(),
            c.transform.trans(pos[0], pos[1]),
            g,
        );
    }
}
