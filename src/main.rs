extern crate piston_window;
extern crate rand;

mod drawing;
mod game;
mod player;
mod space;

use drawing::to_gui_coord_u32;
use game::Game;
use piston_window::types::Color;
use piston_window::*;

const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];
const WINDOW_WIDTH: usize = 1600;
const WINDOW_HEIGHT: usize = 800;

fn main() {
    // Prepare window settings
    let mut window_settings = WindowSettings::new(
        "Doom BSP engine",
        [
            to_gui_coord_u32(WINDOW_WIDTH),
            to_gui_coord_u32(WINDOW_HEIGHT),
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
        if let Some(Button::Keyboard(key)) = event.release_args() {
            game.key_released(key);
        }
        game.player.tick();
        // Draw all of them
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });
    }
}
