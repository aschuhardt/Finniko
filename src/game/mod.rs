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
mod message;

use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use mopa;
pub use uuid::Uuid;
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
pub use self::message::Message;

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

/// The possible results of an attempt by a Movable implementor to
/// move in a particular direction.
enum MovementResult {
    Wall,
    MapEdge([i32; 2]),
    Clear,
}

/// Implemented by structs capable of being moved in a specified direction.
pub trait Movable: mopa::Any {
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
mopafy!(Movable);


/// Implemented by structs representing objects that can be drawn.
/// Anything that can have an in-game sprite needs to implement this.
pub trait Drawable: mopa::Any {
    /// Returns a `String` that corresponds which sprite should be
    /// drawn for the implementation.
    fn sprite_key(&self) -> String;

    /// If overridden, indicates whether the implementor's sprite should
    /// be drawn.
    fn visible(&self) -> bool {
        true
    }
}
mopafy!(Drawable);

/// Stores the current state of the game.
pub struct GameState {
    /// The ID of the actor instance that represents the current player.
    pub player_id: Uuid,

    /// Describes the space in which the game's elements take place.
    pub map: Map,

    /// The Actors (enemies, NPCs, etc.) currently in the map.
    pub actors: HashMap<Uuid, Box<Actor>>,

    /// The Entities (interactive objects, terrain, etc.) currently in the map.
    pub entities: HashMap<Uuid, Entity>,

    /// The items currently present in the map.
    pub items: HashMap<Uuid, Item>,

    /// A queue of messages stored for display to the player.
    pub messages: VecDeque<Message>,

    /// Indicates whether or not to show the message queue to the player.
    pub show_messages: bool,
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
            player_id: Uuid::new_v4(),
            map: map_builder.create(),
            actors: HashMap::<Uuid, Box<Actor>>::new(),
            entities: HashMap::<Uuid, Entity>::new(),
            items: HashMap::<Uuid, Item>::new(),
            messages: VecDeque::<Message>::new(),
            show_messages: true,
        }.add_player()
    }

    fn add_player(mut self) -> GameState {
        self.actors.insert(
            self.player_id,
            Box::new(Player::new(self.player_id)),
        );
        self
    }
}
