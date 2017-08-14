#![deny(missing_docs)]

//! The core state and logic controlling the game.

mod actor;
mod entity;
mod game_controller;
mod game_view;
mod item;
mod map;
mod player;
mod tile;
mod map_builder;
mod texture_mapper;

use std::collections::HashMap;
pub use self::game_controller::GameController;
pub use self::game_view::GameView;
pub use self::actor::Actor;
pub use self::entity::Entity;
pub use self::item::Item;
pub use self::map::Map;
pub use self::tile::Tile;
pub use self::player::Player;
pub use self::map_builder::MapBuilder;
pub use self::texture_mapper::TextureMapper;

/// The width of any given map in number of tiles.
pub const MAP_WIDTH: i32 = 56;

/// The height of any given map in number of tiles.
pub const MAP_HEIGHT: i32 = 32;

/// The range of directions of possible movement.
pub enum MovementDirection {
    /// Indicates that the subject should move to the north.
    Up,

    /// Indicates that the subject should move to the south.
    Down,

    /// Indicates that the subject should move to the west.
    Left,

    /// Indicates that the subject should move to the east.
    Right,

    /// Indicates that the subject should move to the north-west.
    UpLeft,

    /// Indicates that the subject should move to the north-east.
    UpRight,

    /// Indicates that the subject should move to the south-west.
    DownLeft,

    /// Indicates that the subject should move to the south-east.
    DownRight,
}

/// Implemented by structs capable of being moved in a specified direction.
pub trait Movable {
    /// Moves the implementor in the specified direction.
    /// The number of spaces moved is dependent on the properties
    /// of the implementor.
    fn move_toward(&mut self, dir: &MovementDirection);

    /// Returns the current position of the implementor.
    fn current_position(&self) -> [i32; 2];

    /// Sets the X coordinate of the implementor.
    fn set_x(&mut self, x: i32);

    /// Sets the Y coordinate of the implementor.
    fn set_y(&mut self, y: i32);
}

/// Implemented by structs representing objects that can be drawn.
/// Anything that can have an in-game sprite needs to implement this.
pub trait Drawable {
    /// Returns a `String` that corresponds which sprite should be
    /// drawn for the implementation.
    fn get_sprite_key(&self) -> String;
}

/// Implemented by structs capable of posessing an inventory of Items.
pub trait Inventory {
    /// Returns a slice containing references to the items in the implementor's
    /// inventory.
    fn get_items(&self) -> &[Item];
}

/// Stores the current state of the game.
pub struct GameState {
    /// Information about the current state of the player's avatar.
    pub player: Player,

    /// Describes the space in which the game's elements take place.
    pub map: Map,

    /// The Actors (enemies, NPCs, etc.) currently in the map.
    pub actors: HashMap<u16, Actor>,

    /// The Entities (interactive objects, terrain, etc.) currently in the map.
    pub entities: HashMap<u16, Entity>,

    /// The items currently present in the map.
    pub items: HashMap<u16, Item>,
}

/// Helper function for mapping a starting position to a new position given a direction
/// and number of spaces.
fn map_direction_to_position(
    starting_position: [i32; 2],
    dir: &MovementDirection,
    spaces: i32,
) -> [i32; 2] {
    // if the number of spaces was negative, return starting position and log warning
    if spaces < 0 {
        return starting_position;
    }

    // match direction to corresponding change in x, y
    use self::MovementDirection::*;
    let delta = match *dir {
        Up => (0, -1),
        Down => (0, 1),
        Left => (-1, 0),
        Right => (1, 0),
        UpLeft => (-1, -1),
        UpRight => (1, -1),
        DownLeft => (-1, 1),
        DownRight => (1, 1),
    };

    // return deltas multiplied by number of spaces
    [
        starting_position[0] + (delta.0 * spaces),
        starting_position[1] + (delta.1 * spaces),
    ]
}

impl GameState {
    /// Creates and returns a new instance of the GameState struct.
    pub fn new(map_builder: &mut MapBuilder) -> GameState {
        GameState {
            player: Player::new(),
            map: map_builder.create(),
            actors: HashMap::<u16, Actor>::new(),
            entities: HashMap::<u16, Entity>::new(),
            items: HashMap::<u16, Item>::new(),
        }
    }
}
