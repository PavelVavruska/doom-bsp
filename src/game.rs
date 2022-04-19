use crate::space::map;
use crate::space::node::Node;
use piston_window::types::Color;
use piston_window::*;

use super::WINDOW_HEIGHT;
use super::WINDOW_WIDTH;
use drawing::draw_line;

use crate::player::Player;
use crate::space::map::get_tree;

const WORLD_COLOR: Color = [0.1, 0.9, 0.1, 0.3];

pub struct Game {
    // World buffers
    pub frame_buffer: [[bool; WINDOW_HEIGHT]; WINDOW_WIDTH],
    pub frame_buffer_next_tick: [[bool; WINDOW_HEIGHT]; WINDOW_WIDTH],
    player: Player,
    map: Node,
}

impl Game {
    pub fn new() -> Game {
        // randomize world
        let temp_world = [[false; WINDOW_HEIGHT]; WINDOW_WIDTH];

        Game {
            frame_buffer: temp_world,
            frame_buffer_next_tick: [[false; WINDOW_HEIGHT]; WINDOW_WIDTH],
            player: Player::new(100.0, 100.0, 0.0, 100.0),
            map: get_tree(),
        }
    }
    pub fn key_pressed(&self, key: Key) {
        let probability = match key {
            Key::NumPad1 => 0.1, // left down
            Key::NumPad2 => 0.2, // down
            Key::NumPad3 => 0.3, // right down
            Key::NumPad4 => 0.4, // left
            Key::NumPad5 => 0.5, // ?
            Key::NumPad6 => 0.6, // right
            Key::NumPad7 => 0.7, // left up
            Key::NumPad8 => 0.8, // up
            Key::NumPad9 => 0.9, // right up
            _ => 0.5,
        };
    }

    pub fn draw(&mut self, con: &Context, g: &mut G2d) {
        // Iterate over the world

        let game_map_node = map::get_tree();
        let (x, y, z) = game_map_node.travers(&self.player);
        match y {
            None => {}
            Some(line_segments) => {
                for line_segment in line_segments.get_lines() {
                    draw_line(
                        WORLD_COLOR,
                        line_segment.first.x as i32,
                        line_segment.first.y as i32,
                        line_segment.second.x as i32,
                        line_segment.second.y as i32,
                        con,
                        g,
                    );
                }
            }
        }
        let test = 1;
    }

    fn restart_game(self) -> Game {
        Game::new()
    }
}
