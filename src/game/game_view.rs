use graphics::Context;
use opengl_graphics::GlGraphics;
use super::{MAP_WIDTH, MAP_HEIGHT, GameController, TextureMapper, Drawable};

/// Renders information about the game's current state to the screen.
pub struct GameView {
    tm: TextureMapper,
}

impl GameView {
    /// Creates and returns a new instance of the GameView struct.
    pub fn new() -> GameView {
        GameView { tm: TextureMapper::new() }
    }

    /// Renders visuals onto the window based on the state of the provided controller instance.
    pub fn draw(&mut self, controller: &GameController, c: &Context, g: &mut GlGraphics) {
        let screen_rect = c.viewport
            .unwrap_or_else(|| panic!("Could not get the viewport!"))
            .rect;
        let tile_w = screen_rect[2] as f64 / MAP_WIDTH as f64;
        let tile_h = screen_rect[3] as f64 / MAP_HEIGHT as f64;
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
        )
    }
}
