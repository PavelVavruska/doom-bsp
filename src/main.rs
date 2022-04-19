extern crate piston_window;
extern crate rand;

mod drawing;
mod game;
mod player;
mod space;

use piston_window::types::Color;
use piston_window::*;

use drawing::to_gui_coord_u32;
use game::Game;

const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];

const WINDOW_WIDTH: usize = 400;
const WINDOW_HEIGHT: usize = 300;

fn main() {
    // Prepare window settings
    let mut window_settings = WindowSettings::new(
        "Doom BSP engine",
        [
            to_gui_coord_u32(WINDOW_WIDTH as i32),
            to_gui_coord_u32(WINDOW_HEIGHT as i32),
        ],
    )
    .exit_on_esc(true);

    // Fix vsync extension error for linux
    window_settings.set_vsync(true);

    // Create a window
    let mut window: PistonWindow = window_settings.build().unwrap();

    // Create a world
    let mut game = Game::new();

    // Event loop
    while let Some(event) = window.next() {
        // Catch the events of the keyboard
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        // Draw all of them
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
            game.frame_buffer = [[false; WINDOW_HEIGHT]; WINDOW_WIDTH];
            game.frame_buffer
                .copy_from_slice(&game.frame_buffer_next_tick);
            game.frame_buffer_next_tick = [[false; WINDOW_HEIGHT]; WINDOW_WIDTH];
        });
    }
}
