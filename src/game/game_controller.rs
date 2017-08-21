use std::rc::Rc;
use piston::input::GenericEvent;
use status::ControllerStatus;
use rayon::prelude::*;
use uuid::Uuid;
use super::{Drawable, Actor, GameState, MapBuilder};
use super::player::Player;
use super::message::{Message, MessageType};

/// Stores and updates the game's current state.
pub struct GameController {
    state: GameState,
    status: Option<ControllerStatus>,
    map_builder: MapBuilder,
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
            Ok(tile.sprite_key())
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
        let mut working_actors_copy = self.state.actors.clone();
        for (_, actor_rc) in self.state.actors.iter_mut() {
            if let Some(actor) = Rc::get_mut(actor_rc) {
                actor.on_update(&mut working_actors_copy);
            }
            drop(actor_rc);
        }

        if let Some(player_actor_rc) = self.state.actors.get_mut(&self.state.player_id) {
            if let Some(player_actor) = Rc::get_mut(player_actor_rc) {
                if let Some(ref mut player) = player_actor.downcast_mut::<Player>() {
                    player.event_update(event, &self.state.map);
                } else {
                    error!("Player is unable to process events!")
                }
            } else {
                error!("Unable to downcast player actor!")
            }
            drop(player_actor_rc);
        }
    }

    /// Returns the player's current position.
    pub fn player_position(&self) -> [i32; 2] {
        if let Some(player) = self.state.actors.get(&self.state.player_id) {
            let pos = player.current_position();
            drop(player);
            pos
        } else {
            error!("Unable to retrieve player position!");
            [0; 2]
        }
    }

    /// Returns a reference to the current status of the controller, indicating
    /// whether it needs to affect the program flow in some way.
    pub fn get_status(&mut self) -> Option<ControllerStatus> {
        let s = self.status.clone();
        self.status = None;
        s
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

    pub fn actor_sprites(&self) -> Vec<(String, [i32; 2])> {
        let mut sprite_positions = Vec::<(String, [i32; 2])>::new();
        for (_, actor) in self.state.actors.iter().filter(|&(_, v)| v.visible()) {
            sprite_positions.push((actor.sprite_key(), actor.current_position()));
        }
        sprite_positions
    }

    fn add_new_message(&mut self, contents: String, msg_type: MessageType) {
        self.state.messages.push_back(Message {
            contents: contents,
            message_type: msg_type,
        });
    }
}
