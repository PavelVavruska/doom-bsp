use super::WINDOW_HEIGHT;
use super::WINDOW_WIDTH;
use crate::player::Player;
use crate::space::line::calculate_x_y_line_for_x;
use crate::space::map;
use drawing::{draw_line_minimap, draw_rectange, draw_wall_line_first_person};
use piston_window::types::Color;
use piston_window::*;

const WORLD_COLOR: Color = [0.1, 0.9, 0.1, 1.0];
const PORTAL_COLOR: Color = [0.9, 0.0, 0.0, 1.0];
const PLAYER_COLOR: Color = [0.9, 0.9, 0.9, 1.0];
const TRANSFORMED_MINIMAP_X_OFFSET: f64 = 650.0;

pub struct Game {
    // World buffers
    pub frame_buffer: [[bool; WINDOW_HEIGHT]; WINDOW_WIDTH],
    pub frame_buffer_next_tick: [[bool; WINDOW_HEIGHT]; WINDOW_WIDTH],
    player: Player,
}

impl Game {
    pub fn new() -> Game {
        // randomize world
        let temp_world = [[false; WINDOW_HEIGHT]; WINDOW_WIDTH];

        Game {
            frame_buffer: temp_world,
            frame_buffer_next_tick: [[false; WINDOW_HEIGHT]; WINDOW_WIDTH],
            player: Player::new(100.0, 100.0, 0.0, 10.0, 0.0, 50.0),
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

        let mut x_buffer_array: [bool; WINDOW_WIDTH];
        // Draw minimap
        match y {
            None => {}
            Some(line_segments) => {
                for line_segment in line_segments.get_lines() {
                    // minimap view
                    let wall_color = if line_segment.is_portal {
                        PORTAL_COLOR
                    } else {
                        WORLD_COLOR
                    };

                    // transformed minimap view
                    // transform vertexes relative to the player
                    let abs_diff_1_x = line_segment.first.x - self.player.x;
                    let abs_diff_2_x = line_segment.second.x - self.player.x;
                    let abs_diff_1_y = line_segment.first.y - self.player.y;
                    let abs_diff_2_y = line_segment.second.y - self.player.y;

                    // rotate around the player
                    let mut trans_diff_rot_1_x = abs_diff_1_x * self.player.view_angle.cos()
                        + abs_diff_1_y * self.player.view_angle.sin();
                    let mut trans_diff_rot_1_y = -abs_diff_1_x * self.player.view_angle.sin()
                        + abs_diff_1_y * self.player.view_angle.cos();

                    let mut trans_diff_rot_2_x = abs_diff_2_x * self.player.view_angle.cos()
                        + abs_diff_2_y * self.player.view_angle.sin();
                    let mut trans_diff_rot_2_y = -abs_diff_2_x * self.player.view_angle.sin()
                        + abs_diff_2_y * self.player.view_angle.cos();

                    // line clipping behind camera
                    // if not portal:
                    if trans_diff_rot_1_x <= 1.0 && trans_diff_rot_2_x <= 1.0 {
                        // trivial reject
                        continue;
                    } else if trans_diff_rot_1_x <= 1.0 {
                        // 1 x is behind player, calculate
                        let (x1, y1) = calculate_x_y_line_for_x(
                            trans_diff_rot_2_y,
                            trans_diff_rot_1_y,
                            trans_diff_rot_2_x,
                            trans_diff_rot_1_x,
                            2.0,
                        );
                        trans_diff_rot_1_x = x1;
                        trans_diff_rot_1_y = y1;
                    } else if trans_diff_rot_2_x <= 1.0 {
                        // 2 x is behind player, calculate
                        let (x2, y2) = calculate_x_y_line_for_x(
                            trans_diff_rot_2_y,
                            trans_diff_rot_1_y,
                            trans_diff_rot_2_x,
                            trans_diff_rot_1_x,
                            2.0,
                        );
                        trans_diff_rot_2_x = x2;
                        trans_diff_rot_2_y = y2;
                    }

                    let trans_diff_rot_center_1_x = trans_diff_rot_1_x + WINDOW_WIDTH as f64 / 6.0;
                    let trans_diff_rot_center_1_y = trans_diff_rot_1_y + WINDOW_HEIGHT as f64 / 4.0;
                    let trans_diff_rot_center_2_x = trans_diff_rot_2_x + WINDOW_WIDTH as f64 / 6.0;
                    let trans_diff_rot_center_2_y = trans_diff_rot_2_y + WINDOW_HEIGHT as f64 / 4.0;

                    // first person view
                    draw_wall_line_first_person(
                        wall_color,
                        &self.player,
                        trans_diff_rot_1_x,
                        trans_diff_rot_1_y,
                        trans_diff_rot_2_x,
                        trans_diff_rot_2_y,
                        con,
                        g,
                    );

                    draw_line_minimap(
                        wall_color,
                        line_segment.first.x,
                        line_segment.first.y,
                        line_segment.second.x,
                        line_segment.second.y,
                        0.0,
                        0.0,
                        con,
                        g,
                    );

                    draw_line_minimap(
                        wall_color,
                        trans_diff_rot_center_1_x,
                        trans_diff_rot_center_1_y,
                        trans_diff_rot_center_2_x,
                        trans_diff_rot_center_2_y,
                        TRANSFORMED_MINIMAP_X_OFFSET,
                        0.0,
                        con,
                        g,
                    );
                }
            }
        }

        // Draw player in minimap
        draw_rectange(
            PORTAL_COLOR,
            *&self.player.x - 10.0,
            *&self.player.y - 10.0,
            20,
            20,
            con,
            g,
        );
        draw_line_minimap(
            PLAYER_COLOR,
            *&self.player.x + (&self.player.view_angle).cos() * &self.player.move_speed * 2.0,
            *&self.player.y + (&self.player.view_angle).sin() * &self.player.move_speed * 2.0,
            *&self.player.x + (&self.player.view_angle).cos(),
            *&self.player.y + (&self.player.view_angle).sin(),
            0.0,
            0.0,
            con,
            g,
        );

        // draw player in transformed minimap
        draw_rectange(
            PORTAL_COLOR,
            TRANSFORMED_MINIMAP_X_OFFSET + WINDOW_WIDTH as f64 / 6.0 - 10.0,
            WINDOW_HEIGHT as f64 / 4.0 - 10.0,
            20,
            20,
            con,
            g,
        );
    }

    fn restart_game(self) -> Game {
        Game::new()
    }
}
