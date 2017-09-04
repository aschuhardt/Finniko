use std::collections::VecDeque;
use uuid::Uuid;
use bresenham::Bresenham;
use game::actor::{Actor, ActorStatus, ActorType, ActorInfo, BehaviorStyle};
use game;
use game::message::MessageType;
use game::{Message, Movable, MovementDirection, Drawable, Positioned, SpriteInfo};

/// The default number of spaces that the player moves at once.
pub const MOVEMENT_AMOUNT: i32 = 1;

const SPRITE_INFO: SpriteInfo = SpriteInfo {
    key: "mutant",
    color: [0.937, 0.529, 0.0, 1.0],
};

const PERSONAL_SPACE: f32 = 3.0;

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

    fn set_x(&mut self, x: i32) {
        self.position = [x, self.position[1]];
    }

    fn set_y(&mut self, y: i32) {
        self.position = [self.position[0], y];
    }
}

impl Positioned for Soldier {
    fn current_position(&self) -> [i32; 2] {
        self.position
    }
}

impl Drawable for Soldier {
    fn sprite_components(&self) -> SpriteInfo {
        SPRITE_INFO
    }
}

impl Actor for Soldier {
    fn init(&mut self, _: [i32; 2], _: BehaviorStyle) -> Result<Uuid, String> {
        Ok(self.id)
    }

    fn on_create(&mut self) {
        self.messages.push_back(Message {
            contents: format!("Soldier was created!  ID: {}", self.id),
            message_type: MessageType::Danger,
        });
    }

    fn on_update(&mut self, actors: &[ActorInfo]) {
        if let Some(player) = actors.iter().find(|a| a.actor_type == ActorType::Player) {
            let self_pos = (self.position[0], self.position[1]);
            let player_pos = (player.position[0], player.position[1]);
            if let Some(next_pos) = Bresenham::new(self_pos, player_pos).nth(1) {
                if !actors.iter().any(|a| {
                    let distance_from_destination = (((next_pos.0 - a.position[0]).pow(2) +
                                                          (next_pos.1 - a.position[1]).pow(2)) as
                                                         f32)
                        .sqrt();
                    let distance_from_player = (((self.position[0] - a.position[0]).pow(2) +
                                                     (self.position[1] - a.position[1]).pow(2)) as
                                                    f32)
                        .sqrt();
                    if a.actor_type == ActorType::Player {
                        return false;
                    } else {
                        return distance_from_destination <= PERSONAL_SPACE &&
                            distance_from_player > PERSONAL_SPACE;
                    }
                })
                {
                    self.set_x(next_pos.0 as i32);
                    self.set_y(next_pos.1 as i32);
                }
            }
        }
    }

    fn on_interact(&mut self, _: &[ActorInfo]) {}

    fn on_remove(&mut self, _: &[ActorInfo]) {}

    fn actor_type(&self) -> ActorType {
        ActorType::Soldier
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
