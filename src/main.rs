#![deny(missing_docs)]

//! A roguelike game built using the Piston libraries.

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate hyphenation;
extern crate opengl_graphics;
extern crate ndarray;
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate find_folder;
extern crate serde_json;
extern crate texture_coords;
extern crate image;
extern crate texture;
extern crate rand;
extern crate ndarray_parallel;
extern crate noise;
extern crate rayon;
extern crate bmfont;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate mopa;
extern crate textwrap;
extern crate uuid;

mod debug_info;
mod game;
mod text_renderer;

use piston::window::{Window, WindowSettings};
use piston::event_loop::{Events, EventSettings, EventLoop};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use debug_info::{DebugInfoController, DebugInfoView};
use game::{GameController, GameView};
use text_renderer::TextRenderer;

const WINDOW_WIDTH: u32 = 1386;
const WINDOW_HEIGHT: u32 = 792;

lazy_static! {
    static ref TEXT_RENDERER: TextRenderer = TextRenderer::new();
}

/// Contains an Enum used for specifying state control actions by Controllers.
mod status {
    /// Values that can be returned by a Controller in order to affect
    /// the program flow.
    #[derive(Debug, Clone)]
    pub enum ControllerStatus {
        /// Indicates that the returning controller wishes to close
        /// the game window.
        Quit,
        Resize(u32, u32),
    }
}

fn main() {
    // initialize logging
    simple_logger::init().expect("Unable to init logger.");
    info!("Logging initialized.");

    // initialize window settings, events, and graphics
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("finniko", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .vsync(false)
        .opengl(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut event_settings = EventSettings::new();
    event_settings.set_max_fps(65);
    let mut events = Events::new(event_settings);
    let mut gl = GlGraphics::new(opengl);
    info!(
        "Graphical systems initialized...  Using Opengl version {:?}",
        opengl
    );

    // initialize state models

    // Primary game state logic
    let mut game_controller = GameController::new();
    let mut game_view = GameView::new(&TEXT_RENDERER);

    // Debug information meant to aid in troubleshooting and optimization
    let mut debug_controller = DebugInfoController::new(WINDOW_HEIGHT, WINDOW_WIDTH);
    let mut debug_view = DebugInfoView::new(&TEXT_RENDERER);

    while let Some(e) = events.next(&mut window) {
        // pass event reference to controllers
        game_controller.update(&e);
        debug_controller.update(&e);

        // check game controller status
        if let Some(status) = game_controller.get_status() {
            use status::ControllerStatus::*;

            // handle whatever status the controller returned
            match status {
                Quit => {
                    info!("Received Quit status from game_controller, quitting now...");
                    window.set_should_close(true);
                }
                Resize(width, height) => {
                    info!(
                        "Received Resize status from game_controller, resizing to {:?}x{:?}...",
                        width,
                        height
                    );
                    window.window.set_inner_size(width, height);
                }
            }
        }

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                graphics::clear([0.0; 4], g);

                //pass controller reference, context reference, and graphics instance to views
                game_view.draw(&game_controller, &c, g);
                debug_view.draw(&debug_controller, &c, g);

                debug_controller.fps_tick();
            });
        }
    }
}
