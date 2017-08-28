use std::collections::VecDeque;
use uuid::Uuid;
use piston::input::{Button, Key, GenericEvent};
use game;
use game::actor::{Actor, ActorStatus, ActorType, ActorInfo, BehaviorStyle};
use game::message::MessageType;
use game::{Message, Map, Movable, MovementResult, MovementDirection, Drawable};

/// The default number of spaces that the player moves at once.
pub const MOVEMENT_AMOUNT: i32 = 1;

const SPRITE_KEY: &'static str = "pc";
const SPRITE_COLOR: [f32; 4] = [0.141, 0.424, 0.376, 1.0];

/// Stores information and statistics pertaining to the
/// player's avatar.
pub struct Player {
    position: [i32; 2],
    id: Uuid,
    status: Option<ActorStatus>,
    messages: VecDeque<Message>,
    ticks: Option<u32>,
}

impl Player {
    /// Creates and returns a new instance of the Player struct
    pub fn new(id: Uuid) -> Player {
        Player {
            position: [0; 2],
            id: id,
            status: None,
            messages: VecDeque::<Message>::new(),
            ticks: None,
        }
    }

    pub fn ticks(&mut self) -> Option<u32> {
        let ticks = self.ticks;
        self.ticks = None;
        ticks
    }

    /// Allows the Player actor to process input events
    pub fn event_update<G: GenericEvent>(&mut self, event: &G, map: &Map) {
        if let Some(btn) = event.press_args() {
            use game::MovementDirection::*;
            match btn {
                Button::Keyboard(Key::F1) => {
                    self.status = Some(ActorStatus::SpawnActorAt(ActorType::Soldier, [10, 10]));
                }
                Button::Keyboard(Key::Tab) => {
                    self.status = Some(ActorStatus::ToggleMessageVisibility);
                }
                Button::Keyboard(Key::NumPad1) => {
                    self.input_move(map, DownLeft);
                }
                Button::Keyboard(Key::NumPad2) => {
                    self.input_move(map, Down);
                }
                Button::Keyboard(Key::NumPad3) => {
                    self.input_move(map, DownRight);
                }
                Button::Keyboard(Key::NumPad4) => {
                    self.input_move(map, Left);
                }
                Button::Keyboard(Key::NumPad6) => {
                    self.input_move(map, Right);
                }
                Button::Keyboard(Key::NumPad7) => {
                    self.input_move(map, UpLeft);
                }
                Button::Keyboard(Key::NumPad8) => {
                    self.input_move(map, Up);
                }
                Button::Keyboard(Key::NumPad9) => {
                    self.input_move(map, UpRight);
                }
                Button::Keyboard(Key::NumPad0) => {
                    self.perform_ticks(1);
                }
                _ => {}
            }
        }
    }

    fn input_move(&mut self, map: &Map, dir: MovementDirection) {
        match game::try_move(self, map, &dir, MOVEMENT_AMOUNT) {
            MovementResult::Clear => self.move_toward(&dir),
            MovementResult::MapEdge(edge_pos) => {
                self.move_over_edge(map, edge_pos);
                let new_map_offset = Player::get_new_map_offset_from_edge(map, edge_pos);
                self.status = Some(ActorStatus::LoadMapAtRelativeOffset(new_map_offset));
            }
            _ => {}
        }
        self.perform_ticks(1);
    }

    fn perform_ticks(&mut self, count: u32) {
        self.ticks = Some(count);
    }

    fn move_over_edge(&mut self, map: &Map, edge_pos: [i32; 2]) {
        let (edge_x, edge_y) = (edge_pos[0], edge_pos[1]);
        let (width, height) = (map.width() as i32, map.height() as i32);
        if edge_x == -1 {
            self.set_x(width - 1);
        } else if edge_x == width {
            self.set_x(0);
        }
        if edge_y == -1 {
            self.set_y(height - 1);
        } else if edge_y == height {
            self.set_y(0);
        }
    }

    fn get_new_map_offset_from_edge(map: &Map, edge_pos: [i32; 2]) -> [i32; 2] {
        let (mut offset_x, mut offset_y) = (0, 0);
        let (edge_x, edge_y) = (edge_pos[0], edge_pos[1]);
        let (width, height) = (map.width() as i32, map.height() as i32);
        if edge_x == -1 {
            offset_x = -1;
        } else if edge_x == width {
            offset_x = 1;
        }
        if edge_y == -1 {
            offset_y = -1;
        } else if edge_y == height {
            offset_y = 1;
        }
        [offset_x, offset_y]
    }
}

impl Movable for Player {
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

impl Drawable for Player {
    fn sprite_components(&self) -> (&str, [f32; 4]) {
        (SPRITE_KEY, SPRITE_COLOR)
    }
}

impl Actor for Player {
    fn init(&mut self, position: [i32; 2], _: BehaviorStyle) -> Result<Uuid, String> {
        Ok(self.id)
    }

    fn on_create(&mut self) {
        self.messages.push_back(Message {
            contents: String::from("Welcome!"),
            message_type: MessageType::Background,
        });
    }

    fn on_update(&mut self, actors: &[ActorInfo]) {}

    fn on_interact(&mut self, actors: &[ActorInfo]) {}

    fn on_remove(&mut self, actors: &[ActorInfo]) {}

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
