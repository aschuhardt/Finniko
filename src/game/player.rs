use super::{Movable, MovementDirection, Drawable};

/// The default number of spaces that the player moves at once.
pub const MOVEMENT_AMOUNT: i32 = 1;

const SPRITE_KEY: &'static str = "37";

/// Stores information and statistics pertaining to the
/// player's avatar.
pub struct Player {
    /// The current position of the player within the game map.
    /// Represents an X and Y offset indicative of which tile
    /// the player is standing on.
    pub position: [i32; 2],
}

impl Player {
    /// Creates and returns a new instance of the Player struct.
    pub fn new() -> Player {
        Player {
            position: [0; 2],
        }
    }
}

impl Movable for Player {
    fn move_toward(&mut self, dir: &MovementDirection) {
        self.position = super::map_direction_to_position(self.position, dir, MOVEMENT_AMOUNT);
    }

    fn current_position(&self) -> [i32; 2] {
        self.position
    }

    fn set_x(&mut self, x: i32) {
        self.position = [x, self.position[1]];
    }

    fn set_y(&mut self, y: i32) {
        self.position = [self.position[0], y];
    }
}

impl Drawable for Player {
    fn get_sprite_key(&self) -> String {
        String::from(SPRITE_KEY)
    }
}
