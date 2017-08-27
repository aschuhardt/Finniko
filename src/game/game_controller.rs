use std::collections::VecDeque;
use piston::input::GenericEvent;
use status::ControllerStatus;
use rayon::prelude::*;
use super::{Drawable, GameState, MapBuilder};
use super::actor;
use super::actor::{Actor, ActorStatus, ActorInfo};
use super::actors::player::Player;
use super::message::Message;

/// Stores and updates the game's current state.
pub struct GameController {
    state: GameState,
    status: Option<ControllerStatus>,
    map_builder: MapBuilder,
    actions: VecDeque<Action>,
    ticks_to_perform: u32,
}

enum Action {
    Spawn(Box<Actor>),
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
            actions: VecDeque::<Action>::new(),
            ticks_to_perform: 0,
        }
    }

    /// Returns the sprite key for the tile at the specified position
    pub fn tile_sprite_at(&self, position: [i32; 2]) -> Result<(String, [f32; 4]), String> {
        if let Some(tile) = self.state.map.get_at(position) {
            Ok(tile.sprite_components())
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
        self.update_player(event);

        if self.ticks_to_perform > 0 {
            for _ in 0..self.ticks_to_perform {
                self.update_actors();
                self.perform_actions();
            }
            self.ticks_to_perform = 0;
        }
    }

    /// Returns the player's current position.
    pub fn player_position(&self) -> [i32; 2] {
        if let Some(player) = self.state.actors.get(&self.state.player_id) {
            player.current_position()
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

    pub fn actor_sprites(&self) -> Vec<((String, [f32; 4]), [i32; 2])> {
        let mut sprite_positions = Vec::<((String, [f32; 4]), [i32; 2])>::new();
        for (_, actor) in self.state.actors.iter().filter(|&(_, v)| v.visible()) {
            sprite_positions.push((actor.sprite_components(), actor.current_position()));
        }
        sprite_positions
    }

    fn perform_actions(&mut self) {
        for action in self.actions.drain(..) {
            match action {
                Action::Spawn(actor) => {
                    self.state.actors.insert(actor.id(), actor);
                }
            }
        }
    }

    fn update_player<E: GenericEvent>(&mut self, event: &E) {
        // update player
        if let Some(player_actor) = self.state.actors.get_mut(&self.state.player_id) {
            if let Some(ref mut player) = player_actor.downcast_mut::<Player>() {
                player.event_update(event, &self.state.map);
                if let Some(count) = player.ticks() {
                    self.ticks_to_perform = count;
                }
            } else {
                error!("Player is unable to process events!")
            }
        }
    }

    fn update_actors(&mut self) {
        // build cache of actor info
        let mut actor_info = Vec::<ActorInfo>::new();
        for actor in self.state.actors.values() {
            actor_info.push(ActorInfo::new(actor));
        }

        // update actors
        for (_, actor) in self.state.actors.iter_mut() {
            // update
            actor.on_update(&actor_info);

            // retrieve messages
            if let Some(messages) = actor.messages() {
                self.state.messages.append(messages);
            }

            // process status
            if let Some(status) = actor.status() {
                match status {
                    ActorStatus::Resize(size) => {
                        self.status = Some(ControllerStatus::Resize(size[0], size[1]));
                    }
                    ActorStatus::LoadMapAtRelativeOffset(offset) => {
                        self.state.map = self.map_builder.create_offset(offset);
                    }
                    ActorStatus::ToggleMessageVisibility => {
                        self.state.show_messages = !self.state.show_messages;
                    }
                    ActorStatus::SpawnActorAt(actor_type, position) => {
                        let mut spawned = actor::create(actor_type);
                        spawned.set_x(position[0]);
                        spawned.set_y(position[1]);
                        self.actions.push_back(Action::Spawn(spawned));
                    }
                    ActorStatus::Quit => {
                        self.status = Some(ControllerStatus::Quit);
                    }
                }
            }
        }
    }
}
