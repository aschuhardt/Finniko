use std::collections::VecDeque;
use uuid::Uuid;
use piston::input::GenericEvent;
use bresenham::Bresenham;
use status::ControllerStatus;
use super::{Drawable, Movable, Positioned, GameState, MapBuilder, SpriteInfo};
use super::actor;
use super::actor::{Actor, ActorStatus, ActorInfo};
use super::actors::player::Player;
use super::message::Message;

const SPRITE_KEY_VOID: &'static str = "void";

/// Stores and updates the game's current state.
pub struct GameController {
    pub player_position: [i32; 2],
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
            player_position: [-1, -1],
            state: state,
            status: None,
            map_builder: map_builder,
            actions: VecDeque::<Action>::new(),
            ticks_to_perform: 0,
        }
    }

    /// Returns the sprite key for the tile at the specified position
    pub fn tile_sprite_at(&self, position: [i32; 2]) -> Result<SpriteInfo, String> {
        if let Some(tile) = self.state.map.get_at(position) {
            Ok(self.get_sprite_at_distance(
                tile.current_position(),
                tile.sprite_components(),
            ))
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

    pub fn actor_sprites(&self) -> Vec<(SpriteInfo, [i32; 2])> {
        let mut sprite_positions = Vec::<(SpriteInfo, [i32; 2])>::new();
        for actor in self.state.actors.values() {
            let sprite =
                self.get_sprite_at_distance(actor.current_position(), actor.sprite_components());
            sprite_positions.push((sprite, actor.current_position()));
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
        let id = self.state.player_id.clone();
        let ref map = self.state.map.clone();
        let mut ticks_to_add = 0;
        let mut player_position = [-1; 2];

        match self.get_downcasted_actor::<Player>(&id) {
            Ok(player) => {
                player.event_update(event, map);
                player_position = player.current_position();
                if let Some(count) = player.ticks() {
                    ticks_to_add = count;
                }
            }
            Err(why) => {
                error!("{}", why);
            }
        };

        self.ticks_to_perform += ticks_to_add;
        self.player_position = player_position;
    }

    fn get_downcasted_actor<A: Actor + Movable>(&mut self, id: &Uuid) -> Result<&mut A, String> {
        if let Some(actor) = self.state.actors.get_mut(id) {
            if let Some(concrete_actor) = actor.downcast_mut::<A>() {
                Ok(concrete_actor)
            } else {
                Err(format!("Could not cast actor with ID {} from ", id))
            }
        } else {
            Err(format!("Could not find actor with ID {}", id))
        }
    }

    fn update_actors(&mut self) {
        // build cache of actor info
        let mut actor_info = Vec::<ActorInfo>::new();
        for actor in self.state.actors.values() {
            actor_info.push(ActorInfo::new(actor.as_ref()));
        }

        // update actors
        for actor in &mut self.state.actors.values_mut() {
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
                        let mut spawned = actor::create(&actor_type);
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

    fn get_sprite_at_distance(&self, position: [i32; 2], sprite: SpriteInfo) -> SpriteInfo {
        if !self.within_player_view(position) {
            return SpriteInfo {
                key: SPRITE_KEY_VOID,
                color: [0.0; 4],
            };
        }
        sprite
    }

    fn within_player_view(&self, position: [i32; 2]) -> bool {
        self.ray_visible(position, self.player_position)
    }

    fn ray_visible(&self, origin: [i32; 2], target: [i32; 2]) -> bool {
        use super::tile::TileType;
        let origin_point = (origin[0], origin[1]);
        let target_point = (target[0], target[1]);

        let mut ray = Bresenham::new(origin_point, target_point);

        while let Some(coords) = ray.next() {
            if let Some(ray_node) = self.state.map.get_at([coords.0, coords.1]) {
                match ray_node.tile_type {
                    TileType::Wall(_, _) => {
                        return false;
                    }
                    _ => {}
                }
            }
        }
        true
    }
}
