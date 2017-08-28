use graphics::{self, Context};
use opengl_graphics::GlGraphics;
use hyphenation::{self, Language};
use textwrap::Wrapper;
use super::message::*;
use super::{MAP_WIDTH, MAP_HEIGHT, GameController, TextureMapper};
use text_renderer::{FontSize, TextRenderer};

const MESSAGE_LEFT_PAD: f64 = 10.0;
const MESSAGE_VERTICAL_ADJUSTMENT: f64 = -6.0;
const TEXT_WRAP_WIDTH: usize = 32;
const MESSAGE_DISPLAY_WIDTH: f64 = 250.0;

/// Renders information about the game's current state to the screen.
pub struct GameView {
    tm: TextureMapper,
    text_renderer: &'static TextRenderer,
    text_wrapper: Wrapper<'static>,
}

impl GameView {
    /// Creates and returns a new instance of the GameView struct.
    pub fn new(txt: &'static TextRenderer) -> GameView {
        let corpus = hyphenation::load(Language::English_US).unwrap();
        GameView {
            tm: TextureMapper::new(),
            text_renderer: txt,
            text_wrapper: Wrapper::new(TEXT_WRAP_WIDTH).word_splitter(Box::new(corpus)),
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
                    Ok(sprite) => {
                        self.tm.draw_at(
                            [
                                x as f64 * tile_w,
                                y as f64 * tile_h,
                                tile_w + 1.0,
                                tile_h + 1.0,
                            ],
                            sprite.key,
                            sprite.color,
                            c.transform,
                            g,
                        );
                    }
                    Err(why) => error!("{:?}", why),
                };
            }
        }

        // draw actors (this includes the player)
        for (sprite, position) in controller.actor_sprites() {
            self.tm.draw_at(
                [
                    position[0] as f64 * tile_w,
                    position[1] as f64 * tile_h,
                    tile_w,
                    tile_h,
                ],
                sprite.key,
                sprite.color,
                c.transform,
                g,
            );
        }

        // draw message
        if controller.should_show_messages() {
            // background
            let screen_w = screen_rect[2] as f64;
            let screen_h = screen_rect[3] as f64;

            let line_height = self.text_renderer.line_height(FontSize::Size24) as f64;

            // check which side of the screen the player is on,
            // and adjust the position of message box accordingly
            let mut left_adjust = 0.0;
            let player_x = controller.player_position()[0] as f64;
            if player_x * tile_w < screen_w / 2.0 {
                left_adjust = screen_w - MESSAGE_DISPLAY_WIDTH;
            }

            graphics::rectangle(
                [0.1, 0.1, 0.1, 0.9],
                [left_adjust, 0.0, MESSAGE_DISPLAY_WIDTH, screen_h],
                c.transform,
                g,
            );

            // draw messages
            let line_count = ((screen_h / 2.0) / line_height) as usize - 1;
            let messages = controller.get_messages(line_count);
            let mut lines = 0usize;
            let mut msg_index = 0usize;
            'outer: while lines < line_count && msg_index < messages.len() {
                let msg = &messages[msg_index];
                let mut wrapped_text = String::from("**");
                wrapped_text.push_str(self.text_wrapper.fill(msg.contents.as_str()).as_str());

                for line in wrapped_text.split('\n').rev() {
                    let color = match msg.message_type {
                        MessageType::Normal => MESSAGE_COLOR_NORMAL,
                        MessageType::Danger => MESSAGE_COLOR_DANGER,
                        MessageType::Benefit => MESSAGE_COLOR_BENEFIT,
                        MessageType::Background => MESSAGE_COLOR_BACKGROUND,
                    };
                    let position = [
                        MESSAGE_LEFT_PAD + left_adjust,
                        (line_height * line_count as f64) - (line_height * lines as f64) +
                            (screen_h / 2.0) +
                            MESSAGE_VERTICAL_ADJUSTMENT,
                    ];

                    self.text_renderer.draw_at(
                        position,
                        String::from(line),
                        c.transform,
                        FontSize::Size18,
                        color,
                        g,
                    );

                    lines += 1;
                    if lines >= line_count {
                        break 'outer;
                    }
                }

                msg_index += 1;
            }
        }
    }
}
