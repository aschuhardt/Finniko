use piston::input::{Button, Key, GenericEvent};
use debug_info::DebugInfo;

/// Implements logic related to collecting and storing
/// debugging and troubleshooting information.
pub struct DebugInfoController {
    /// The controller's state.  Stores the information used by the controller
    /// and view to collect and display debug statistics.
    state: DebugInfo,
}

impl DebugInfoController {
    /// Creates and returns a new instance of the DebugInfoController struct
    /// with a new state object set to store the provided integers in its
    /// window size fields.
    pub fn new(window_w: u32, window_h: u32) -> DebugInfoController {
        DebugInfoController::new_with(DebugInfo::new(window_w, window_h))
    }

    /// Creates and returns a new instance of the DebugInfoController struct
    /// with the provided state.
    pub fn new_with(state: DebugInfo) -> DebugInfoController {
        DebugInfoController { state: state }
    }

    /// Returns the current window size.
    pub fn window_size(&self) -> &[u32; 2] {
        &self.state.window_size
    }

    /// Returns the difference in the mouse's position between
    /// this update and the previous.
    pub fn mouse_relative_pos(&self) -> &[f64; 2] {
        &self.state.mouse_relative
    }

    /// Returns the position of the mouse in the window.
    pub fn mouse_window_pos(&self) -> &[f64; 2] {
        &self.state.mouse_window
    }

    /// Returns the mouse's scroll amount.
    pub fn mouse_scroll(&self) -> &[f64; 2] {
        &self.state.mouse_scroll
    }

    /// Returns a value indicating whether the debug information
    /// should be drawn to the window.
    pub fn should_draw(&self) -> &bool {
        &self.state.should_draw
    }

    /// Returns the current framerate.
    pub fn framerate(&self) -> u32 {
        self.state.fps_counter.framerate()
    }

    /// Updates the framerate counter.
    pub fn fps_tick(&mut self) {
        self.state.fps_counter.tick();
    }

    /// Updates the state of the DebugInfoController based on the provided
    /// event information.
    pub fn update<E>(&mut self, event: &E)
    where
        E: GenericEvent,
    {
        if let Some(size) = event.resize_args() {
            self.state.window_size = size;
        } else if let Some(rel) = event.mouse_relative_args() {
            self.state.mouse_relative = rel;
        } else if let Some(pos) = event.mouse_cursor_args() {
            self.state.mouse_window = pos;
        } else if let Some(amt) = event.mouse_scroll_args() {
            self.state.mouse_scroll = amt;
        } else if let Some(btn) = event.press_args() {
            if let Button::Keyboard(Key::F3) = btn {
                self.state.should_draw = !self.state.should_draw;
            }
        }
    }
}
