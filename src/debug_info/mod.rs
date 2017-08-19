#![deny(missing_docs)]

//! Functionality used to track, store, and display information pertaining to the
//! status and performance of the game.

mod debug_info_controller;
mod debug_info_view;
mod fps_counter;

pub use self::debug_info_controller::DebugInfoController;
pub use self::debug_info_view::DebugInfoView;
pub use self::fps_counter::FpsCounter;

/// Stores the current state of information used to track game performance
/// and statistics.
pub struct DebugInfo {
    /// The size of the window.
    pub window_size: [u32; 2],

    /// The position of the mouse cursor in the window.
    pub mouse_window: [f64; 2],

    /// The position of the mouse cursor in the window relative to its location
    /// during the previous update.
    pub mouse_relative: [f64; 2],

    /// The the amount that the mouse's scroll-wheel has turned.
    pub mouse_scroll: [f64; 2],

    /// Whether or not to render the debug information onto the window.
    pub should_draw: bool,

    /// Keeps track of current framerate.
    pub fps_counter: FpsCounter,
}

impl DebugInfo {
    /// Creates and returns a new instance of the DebugInfo struct,
    /// storing the provided integers as initial values in the
    /// window_size field.
    pub fn new(window_w: u32, window_h: u32) -> DebugInfo {
        DebugInfo {
            window_size: [window_w, window_h],
            mouse_window: [0f64, 0f64],
            mouse_relative: [0f64, 0f64],
            mouse_scroll: [0f64, 0f64],
            should_draw: cfg!(debug_assertions),
            fps_counter: FpsCounter::new(),
        }
    }
}
