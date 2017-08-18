/// Normal message color
pub const MESSAGE_COLOR_NORMAL: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

/// Danger message color
pub const MESSAGE_COLOR_DANGER: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

/// Benefit message color
pub const MESSAGE_COLOR_BENEFIT: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

/// Background message color
pub const MESSAGE_COLOR_BACKGROUND: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

/// Represents different styles of message, indicating how text should
/// be displayed.
pub enum MessageType {
    Normal,
    Danger,
    Benefit,
    Background,
}

/// A message that can be displayed to the player.
pub struct Message {
    /// The contents of the message
    pub contents: String,

    /// How the message should be displayed to the player
    pub message_type: MessageType,
}
