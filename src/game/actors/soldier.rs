use std::collections::VecDeque;
use uuid::Uuid;
use game::actor::{Actor, ActorStatus, ActorType, ActorInfo, BehaviorStyle};
use game;
use game::message::MessageType;
use game::{Message, Movable, MovementDirection, Drawable};

/// The default number of spaces that the player moves at once.
pub const MOVEMENT_AMOUNT: i32 = 1;

const SPRITE_KEY: &'static str = "65";

/// Stores information and statistics pertaining to the
/// player's avatar.
pub struct Soldier {
    position: [i32; 2],
    id: Uuid,
    status: Option<ActorStatus>,
    messages: VecDeque<Message>,
}

impl Soldier {
    /// Creates and returns a new instance of the Player struct
    pub fn new() -> Soldier {
        Soldier {
            position: [0; 2],
            id: Uuid::new_v4(),
            status: None,
            messages: VecDeque::<Message>::new(),
        }
    }
}

impl Movable for Soldier {
    fn move_toward(&mut self, dir: &MovementDirection) {
        self.position = game::map_direction_to_position(self.position, dir, MOVEMENT_AMOUNT);
    }

    fn current_position(&self) -> [i32; 2] {
        self.position
    }

    fn set_x(&mut self, x: i32) {
        self.position = [x, self.position[1]];
    }

    fn set_y(&mut self, y: i32) {
        self.position = [self.position[0], y];
    }
}

impl Drawable for Soldier {
    fn sprite_key(&self) -> String {
        String::from(SPRITE_KEY)
    }
}

impl Actor for Soldier {
    fn init(&mut self, position: [i32; 2], _: BehaviorStyle) -> Result<Uuid, String> {
        Ok(self.id)
    }

    fn on_create(&mut self) {
        self.messages.push_back(Message {
            contents: format!("Soldier was created!  ID: {}", self.id),
            message_type: MessageType::Danger,
        });
    }

    fn on_update(&mut self, actors: &Vec<ActorInfo>) {}

    fn on_interact(&mut self, actors: &Vec<ActorInfo>) {}

    fn on_remove(&mut self, actors: &Vec<ActorInfo>) {}

    fn actor_type(&self) -> ActorType {
        ActorType::Player
    }

    fn id(&self) -> Uuid {
        self.id
    }

    fn status(&mut self) -> Option<ActorStatus> {
        let status = self.status.clone();
        self.status = None;
        status
    }

    fn messages(&mut self) -> Option<&mut VecDeque<Message>> {
        Some(&mut self.messages)
    }
}
