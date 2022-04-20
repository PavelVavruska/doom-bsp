use std::f64::consts::PI;

use crate::player;
use crate::space::map;
use crate::space::node::Node;
use crate::space::vec2d::Vec2d;
use piston_window::types::Color;
use piston_window::*;

use super::WINDOW_HEIGHT;
use super::WINDOW_WIDTH;
use drawing::{draw_line, draw_rectange};

use crate::player::Player;
use crate::space::map::get_tree;

const WORLD_COLOR: Color = [0.1, 0.9, 0.1, 0.3];
const PORTAL_COLOR: Color = [0.9, 0.0, 0.0, 0.5];
const WORLD_COLOR_DARK: Color = [0.1, 0.3, 0.1, 0.3];
const PORTAL_COLOR_DARK: Color = [0.3, 0.0, 0.0, 0.5];
const PLAYER_COLOR: Color = [0.9, 0.9, 0.9, 0.5];

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
            player: Player::new(100.0, 100.0, 0.0, 100.0, 0.0, 50.0),
            map: get_tree(),
        }
    }
    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::Up => self.player.move_forward(),
            Key::W => self.player.move_forward(),
            Key::Down => self.player.move_backward(),
            Key::S => self.player.move_backward(),
            Key::Left => self.player.turn_left(),
            Key::A => self.player.turn_left(),
            Key::Right => self.player.turn_right(),
            Key::D => self.player.turn_right(),
            _ => {}
        };
    }

    /// Draws entire world.
    pub fn draw(&mut self, con: &Context, g: &mut G2d) {
        // Iterate over the world
        let game_map_node = map::get_tree();
        let (x, y, z) = game_map_node.travers(&self.player);

        // Draw minimap
        match y {
            None => {}
            Some(line_segments) => {
                for line_segment in line_segments.get_lines() {
                    let is_player_watching = line_segment.normal.dot_product_with(
                        Vec2d::new(self.player.view_angle.sin(), self.player.view_angle.cos())
                            .normalize(),
                    );
                    let wall_color = if z == line_segment {
                        if is_player_watching < 0.0 {
                            PORTAL_COLOR
                        } else {
                            PORTAL_COLOR_DARK
                        }
                    } else {
                        if is_player_watching < 0.0 {
                            WORLD_COLOR
                        } else {
                            WORLD_COLOR_DARK
                        }
                    };
                    draw_line(
                        wall_color,
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

        // Draw player in minimap
        draw_rectange(
            PORTAL_COLOR,
            *&self.player.x as i32 - 10,
            *&self.player.y as i32 - 10,
            20,
            20,
            con,
            g,
        );
        draw_line(
            PLAYER_COLOR,
            (*&self.player.x + (&self.player.view_angle).sin() * &self.player.move_speed) as i32,
            (*&self.player.y + (&self.player.view_angle).cos() * &self.player.move_speed) as i32,
            (*&self.player.x + (&self.player.view_angle - PI).sin()) as i32,
            (*&self.player.y + (&self.player.view_angle - PI).cos()) as i32,
            con,
            g,
        );
    }

    fn restart_game(self) -> Game {
        Game::new()
    }
}
