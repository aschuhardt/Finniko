use std::collections::VecDeque;
use mopa;
use uuid::Uuid;
use super::{Drawable, Message, Movable, Positioned};
use super::actors::*;

/// Dictates which set of behavior patterns the actor will exhibit
#[derive(Debug, Clone)]
pub enum BehaviorStyle {
    Friendly,
    Oblivious,
    Hostile,
    Fearful,
    Inactive,
}

/// Markers for the various types of actors that are available
#[derive(Debug, Clone, PartialEq)]
pub enum ActorType {
    Player,
    Soldier,
    // Scripted,
}

/// Used by Actor implementations to force their controller to perform
/// some action
#[derive(Debug, Clone)]
pub enum ActorStatus {
    Resize([u32; 2]),
    LoadMapAtRelativeOffset([i32; 2]),
    ToggleMessageVisibility,
    SpawnActorAt(ActorType, [i32; 2]),
    Quit,
}

/// Used as a means of communicating information about actors to other actors.
#[derive(Debug)]
pub struct ActorInfo {
    pub id: Uuid,
    pub actor_type: ActorType,
    pub position: [i32; 2],
}

impl ActorInfo {
    /// Creates and returns a new instance of the ActorInfo struct from the
    /// information in the provided Actor.
    pub fn new(actor: &Actor) -> ActorInfo {
        ActorInfo {
            id: actor.id(),
            actor_type: actor.actor_type(),
            position: actor.current_position(),
        }
    }
}

/// Stores information pertaining to a single actor in the game's
/// current state.
////
///Examples of actors include enemies, NPCs, creatures, etc.
pub trait Actor: mopa::Any + Drawable + Movable + Positioned {
    /// Initializes the actor and returns its new ID
    fn init(&mut self, position: [i32; 2], behavior: BehaviorStyle) -> Result<Uuid, String>;

    /// Called when the object is created, after it is initialized
    fn on_create(&mut self);

    /// Called on each update tick
    fn on_update(&mut self, actors: &[ActorInfo]);

    /// Called when interacted with by another Actor
    fn on_interact(&mut self, actors: &[ActorInfo]);

    /// Called before the Actor is removed from game state
    fn on_remove(&mut self, actors: &[ActorInfo]);

    /// Returns a marker indicating what type of actor this is
    fn actor_type(&self) -> ActorType;

    /// Returns the numerical ID of the Actor
    fn id(&self) -> Uuid;

    /// Returns the implementor's current `ActorStatus` if one exists
    fn status(&mut self) -> Option<ActorStatus> {
        None
    }

    /// Returns the implementor's queue of messages
    fn messages(&mut self) -> Option<&mut VecDeque<Message>> {
        None
    }
}
mopafy!(Actor);

pub fn create(actor_type: &ActorType) -> Box<Actor> {
    let mut actor: Box<Actor> = match *actor_type {
        ActorType::Player => Box::new(player::Player::new(Uuid::new_v4())),
        ActorType::Soldier => Box::new(soldier::Soldier::new()),
    };
    actor.on_create();
    actor
}
