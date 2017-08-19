use piston::input::{Button, Key, GenericEvent};
use status::ControllerStatus;
use super::{Movable, Drawable, MovementDirection, GameState, MapBuilder};
use super::tile::TileType;
use super::player::{self, Player};
use super::message::{Message, MessageType};

/// Stores and updates the game's current state.
pub struct GameController {
    state: GameState,
    status: Option<ControllerStatus>,
    map_builder: MapBuilder,
}

// /// Represents the menu states that the controller can be in.
// enum MenuState {
//     /// No menu is open.
//     Closed,

//     /// The user has paused the game with the Escape key.
//     Paused,

//     /// The user has opened their character's inventory.
//     Inventory,

//     /// The user has opened the in-game options.
//     Options,
// }

enum MovementResult {
    Wall,
    MapEdge([i32; 2]),
    Clear,
}

impl GameController {
    /// Creates and returns an instance of the GameController struct with
    /// a default state.
    pub fn new() -> GameController {
        let mut map_builder = MapBuilder::new();
        GameController::new_with(GameState::new(&mut map_builder), map_builder)
    }

    /// Creates and returns an instance of the GameController struct with
    /// its `state` field containing the provided GameState instance.
    pub fn new_with(state: GameState, map_builder: MapBuilder) -> GameController {
        GameController {
            state: state,
            status: None,
            map_builder: map_builder,
        }
    }

    /// Returns the sprite key for the tile at the specified position
    pub fn tile_sprite_at(&self, position: [i32; 2]) -> Result<String, String> {
        if let Some(tile) = self.state.map.get_at(position) {
            Ok(tile.get_sprite_key())
        } else {
            Err(format!(
                "Unable to gather sprite key for tile at {:?}",
                position
            ))
        }
    }

    /// Indicates to the view whether or not it should display the message
    /// queue to the player
    pub fn should_show_messages(&self) -> bool {
        self.state.show_messages
    }

    /// Performs game logic routines and alters the state of the controller
    /// based on events received by the window.
    pub fn update<E>(&mut self, event: &E)
    where
        E: GenericEvent,
    {
        if let Some(btn) = event.press_args() {
            use super::MovementDirection::*;
            match btn {
                Button::Keyboard(Key::F1) => {
                    self.status = Some(ControllerStatus::Resize(640u32, 480u32));
                }
                Button::Keyboard(Key::Tab) => {
                    self.state.show_messages = !self.state.show_messages;
                }
                Button::Keyboard(Key::NumPad1) => self.move_player(DownLeft),
                Button::Keyboard(Key::NumPad2) => self.move_player(Down),
                Button::Keyboard(Key::NumPad3) => self.move_player(DownRight),
                Button::Keyboard(Key::NumPad4) => self.move_player(Left),
                Button::Keyboard(Key::NumPad5) => {}
                Button::Keyboard(Key::NumPad6) => self.move_player(Right),
                Button::Keyboard(Key::NumPad7) => self.move_player(UpLeft),
                Button::Keyboard(Key::NumPad8) => self.move_player(Up),
                Button::Keyboard(Key::NumPad9) => self.move_player(UpRight),
                _ => {}
            }
        }
    }

    /// Returns a reference to the current status of the controller, indicating
    /// whether it needs to affect the program flow in some way.
    pub fn get_status(&mut self) -> Option<ControllerStatus> {
        let s = self.status.clone();
        self.status = None;
        s
    }

    /// Returns a reference to the Player object stored in the controller's state.
    pub fn get_player(&self) -> &Player {
        &self.state.player
    }

    /// Returns a collection of messages
    pub fn get_messages(&self, count: usize) -> Vec<Message> {
        self.state
            .messages
            .clone()
            .into_iter()
            .rev()
            .take(count)
            .collect()
    }

    /// Checks to see whether the target is capable of moving in the indicated
    /// direction.
    fn try_move<M>(&self, target: &M, dir: &MovementDirection) -> MovementResult
    where
        M: Movable,
    {
        let check_position = super::map_direction_to_position(
            target.current_position(),
            dir,
            player::MOVEMENT_AMOUNT,
        );
        if let Some(tile) = self.state.map.get_at(check_position) {
            if let TileType::Wall(_, _) = tile.tile_type {
                MovementResult::Wall
            } else {
                MovementResult::Clear
            }
        } else {
            MovementResult::MapEdge(check_position)
        }
    }

    /// Moves the player in a specified direction, then checks whether
    /// it is necessary to transition to a new map.
    fn move_player(&mut self, dir: MovementDirection) {
        match self.try_move(&self.state.player, &dir) {
            MovementResult::Clear => self.state.player.move_toward(&dir),
            MovementResult::MapEdge(edge_pos) => self.handle_edge_movement(edge_pos),
            _ => {}
        }
        let new_position = self.state.player.position;
        self.add_new_message(
            format!("You moved to position {:?}.", new_position),
            MessageType::Normal,
        );
    }

    fn handle_edge_movement(&mut self, edge_pos: [i32; 2]) {
        let (mut offset_x, mut offset_y) = (0, 0);
        let (edge_x, edge_y) = (edge_pos[0], edge_pos[1]);
        let (width, height) = (
            self.state.map.width() as i32,
            self.state.map.height() as i32,
        );
        if edge_x == -1 {
            self.state.player.set_x(width - 1);
            offset_x = -1;
        } else if edge_x == width {
            self.state.player.set_x(0);
            offset_x = 1;
        }
        if edge_y == -1 {
            self.state.player.set_y(height - 1);
            offset_y = -1;
        } else if edge_y == height {
            self.state.player.set_y(0);
            offset_y = 1;
        }
        self.state.map = self.map_builder.create_offset([offset_x, offset_y]);
    }

    fn add_new_message(&mut self, contents: String, msg_type: MessageType) {
        self.state.messages.push_back(Message {
            contents: contents,
            message_type: msg_type,
        });
    }
}
