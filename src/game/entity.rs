use mopa;
use super::{Movable, Drawable, Map, Positioned};
use super::actor::ActorInfo;

/// Things that implement Entity are those which exist in the world but are not
/// "alive".
////
/// Examples include static props and areas of effect.
pub trait Entity: mopa::Any + Movable + Positioned + Drawable {
    fn on_create(&mut self, map: &Map, actors: &[ActorInfo]);
}
mopafy!(Entity);
