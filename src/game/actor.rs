use std::rc::Rc;
use std::any::Any;
use std::collections::{VecDeque, HashMap};
use mopa;
use uuid::Uuid;
use super::{Drawable, Message, Movable, MovementDirection};

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
#[derive(Debug, Clone)]
pub enum ActorType {
    Player,
    Soldier,
}

/// Used by Actor implementations to force their controller to perform
/// some action
#[derive(Debug, Clone)]
pub enum ActorStatus {
    Resize([u32; 2]),
    LoadMapAtRelativeOffset([i32; 2]),
    ToggleMessageVisibility,
}

/// Stores information pertaining to a single actor in the game's
/// current state.
////
///Examples of actors include enemies, NPCs, creatures, etc.
pub trait Actor: mopa::Any + Drawable + Movable {
    /// Initializes the actor and returns its new ID
    fn init(&mut self, position: [i32; 2], behavior: BehaviorStyle) -> Result<Uuid, String>;

    /// Called when the object is created, after it is initialized
    fn on_create(&mut self, actors: &mut HashMap<Uuid, Rc<Actor>>);

    /// Called on each update tick
    fn on_update(&mut self, actors: &mut HashMap<Uuid, Rc<Actor>>);

    /// Called when interacted with by another Actor
    fn on_interact(&mut self, actors: &mut HashMap<Uuid, Rc<Actor>>);

    /// Called before the Actor is removed from game state
    fn on_remove(&mut self, actors: &mut HashMap<Uuid, Rc<Actor>>);

    /// Returns a marker indicating what type of actor this is
    fn actor_type(&mut self) -> ActorType;

    /// Returns the numerical ID of the Actor
    fn id(&mut self) -> Uuid;

    /// Returns the implementor's current `ActorStatus` if one exists
    fn status(&mut self) -> Option<ActorStatus> {
        None
    }

    fn messages(&mut self) -> Option<&VecDeque<Message>> {
        None
    }
}
mopafy!(Actor);
