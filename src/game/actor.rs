use std::rc::Rc;
use std::any::Any;
use mopa;
use super::{Drawable, Movable, MovementDirection};

/// Dictates which set of behavior patterns the actor will exhibit
#[derive(Debug)]
pub enum BehaviorStyle {
    Friendly,
    Oblivious,
    Hostile,
    Fearful,
    Inactive,
}

/// Markers for the various types of actors that are available
#[derive(Debug)]
pub enum ActorType {
    Player,
    Soldier,
    Mercenary,
    Android,
}

/// Stores information pertaining to a single actor in the game's
/// current state.
////
///Examples of actors include enemies, NPCs, creatures, etc.
pub trait Actor: mopa::Any + Drawable + Movable {
    /// Initializes the actor and returns its new ID
    fn init(&mut self, position: [i32; 2], behavior: BehaviorStyle) -> Result<u16, String>;

    /// Checks whether the Actor's `on_create` function has been called
    fn is_new(&self);

    fn move_direction(&self) -> Option<MovementDirection>;

    /// Called when the object is created, after it is initialized
    fn on_create(&mut self);

    /// Called on each update tick
    fn on_update(&mut self);

    /// Called when interacted with by another Actor
    fn on_interact(&mut self);

    /// Called before the Actor is removed from game state
    fn on_remove(&mut self);

    /// Returns a marker indicating what type of actor this is
    fn actor_type(&mut self) -> ActorType;

    fn id(&mut self) -> u16;
}
mopafy!(Actor);

pub fn downcast<A: Actor + Any, F>(actor: &mut Rc<Actor>, mut op: F)
where
    F: FnMut(&mut A),
{
    if let Some(a) = Rc::get_mut(actor) {
        let actor_type = a.actor_type();
        let id = a.id();
        if let Some(ref mut concrete) = a.downcast_mut::<A>() {
            op(concrete);
        } else {
            error!(
                "Could not downcast Actor (type marker: {:?}) with ID {:?}",
                actor_type,
                id
            )
        }
    }
}
