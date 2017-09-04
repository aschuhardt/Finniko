// use std::collections::VecDeque;
// use uuid::Uuid;
// use bresenham::Bresenham;
// use game::actor::{Actor, ActorStatus, ActorType, ActorInfo, BehaviorStyle};
// use game;
// use game::message::MessageType;
// use game::{Message, Movable, MovementDirection, Drawable, SpriteInfo};

// /// The default number of spaces that the player moves at once.
// pub const MOVEMENT_AMOUNT: i32 = 1;

// /// Stores information and statistics pertaining to the
// /// player's avatar.
// pub struct ScriptedActor {
//     position: [i32; 2],
//     id: Uuid,
//     status: Option<ActorStatus>,
//     messages: VecDeque<Message>,
// }

// impl ScriptedActor {
//     /// Creates and returns a new instance of the Player struct
//     pub fn new() -> ScriptedActor {
//         ScriptedActor {
//             position: [0; 2],
//             id: Uuid::new_v4(),
//             status: None,
//             messages: VecDeque::<Message>::new(),
//         }
//     }
// }

// impl Movable for ScriptedActor {
//     fn move_toward(&mut self, dir: &MovementDirection) {
//         self.position = game::map_direction_to_position(self.position, dir, MOVEMENT_AMOUNT);
//     }

//     fn current_position(&self) -> [i32; 2] {
//         self.position
//     }

//     fn set_x(&mut self, x: i32) {
//         self.position = [x, self.position[1]];
//     }

//     fn set_y(&mut self, y: i32) {
//         self.position = [self.position[0], y];
//     }
// }

// impl Drawable for ScriptedActor {
//     fn sprite_components(&self) -> SpriteInfo {
//         SpriteInfo {
//             key: "asdf",

//         }
//     }
// }

// impl Actor for ScriptedActor {
//     fn init(&mut self, position: [i32; 2], _: BehaviorStyle) -> Result<Uuid, String> {
//         Ok(self.id)
//     }

//     fn on_create(&mut self) {

//     }

//     fn on_update(&mut self, actors: &[ActorInfo]) {

//     }

//     fn on_interact(&mut self, actors: &[ActorInfo]) {

//     }

//     fn on_remove(&mut self, actors: &[ActorInfo]) {}

//     fn actor_type(&self) -> ActorType {
//         ActorType::Scripted
//     }

//     fn id(&self) -> Uuid {
//         self.id
//     }

//     fn status(&mut self) -> Option<ActorStatus> {
//         let status = self.status.clone();
//         self.status = None;
//         status
//     }

//     fn messages(&mut self) -> Option<&mut VecDeque<Message>> {
//         Some(&mut self.messages)
//     }
// }
