use std::f64::consts::PI;

use piston_window::line;
use piston_window::rectangle;
use piston_window::types::Color;
use piston_window::Context;
use piston_window::G2d;

use crate::player::Player;
use crate::WINDOW_WIDTH;

const BLOCK_SIZE: f64 = 1.0;
const MINIMAP_HEIGHT: i32 = 400;

pub fn to_gui_coord(game_coord: f64) -> f64 {
    (game_coord) * BLOCK_SIZE
}

pub fn to_gui_coord_u32(game_coord: usize) -> u32 {
    to_gui_coord(game_coord as f64) as u32
}

pub fn draw_rectange(
    color: Color,
    start_x: f64,
    start_y: f64,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let gui_start_x = to_gui_coord(start_x);
    let gui_start_y = to_gui_coord(start_y);

    rectangle(
        color,
        [
            gui_start_x,
            gui_start_y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}

pub fn draw_line_minimap(
    color: Color,
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
    offset_x: f64,
    offset_y: f64,
    con: &Context,
    g: &mut G2d,
) {
    let gui_start_x = to_gui_coord(start_x + offset_x);
    let gui_start_y = to_gui_coord(start_y + offset_y);
    let gui_end_x = to_gui_coord(end_x + offset_x);
    let gui_end_y = to_gui_coord(end_y + offset_y);

    line(
        color,
        2.0,
        [gui_start_x, gui_start_y, gui_end_x, gui_end_y],
        con.transform,
        g,
    );
}

pub fn draw_wall_line_first_person(
    color: Color,
    player: &Player,
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
    con: &Context,
    g: &mut G2d,
) {
    let mut angle_point_start = 0.0;
    if -start_x != 0.0 {
        angle_point_start = ((-start_y) / (-start_x)).atan();
        if angle_point_start > PI {
            angle_point_start -= 2.0 * PI;
        }
        if angle_point_start < PI {
            angle_point_start += 2.0 * PI;
        }
    }
    let mut angle_point_end = 0.0;
    if -end_x != 0.0 {
        angle_point_end = ((-end_y) / (-end_x)).atan();
        if angle_point_end > PI {
            angle_point_end -= 2.0 * PI;
        }
        if angle_point_end < PI {
            angle_point_end += 2.0 * PI;
        }
    }
    let angle_diff_start = -angle_point_start;
    let angle_diff_end = -angle_point_end;

    let distance_from_player_start = start_x;
    let distance_from_player_end = end_x;

    let half_player_fov = player.fov / 2.0;
    let one_pixel_in_rad = half_player_fov / WINDOW_WIDTH as f64;
    let start_angle_to_fov = 800.0 + angle_diff_start.tan() / -one_pixel_in_rad;
    let end_angle_to_fov = 800.0 + angle_diff_end.tan() / -one_pixel_in_rad;

    let gui_start_x = to_gui_coord(start_angle_to_fov);

    let gui_end_x = to_gui_coord(end_angle_to_fov);

    let gui_start_y_top =
        to_gui_coord(MINIMAP_HEIGHT as f64 + 14000.0 / distance_from_player_start); // lenght from player
    let gui_end_y_top = to_gui_coord(MINIMAP_HEIGHT as f64 + 14000.0 / distance_from_player_end);

    let gui_start_y_bottom =
        to_gui_coord(MINIMAP_HEIGHT as f64 - 14000.0 / distance_from_player_start); // lenght from player
    let gui_end_y_bottom = to_gui_coord(MINIMAP_HEIGHT as f64 - 14000.0 / distance_from_player_end);

    let delta_y_1 = (gui_start_y_bottom - gui_start_y_top) / 20.0;
    let delta_y_2 = (gui_end_y_bottom - gui_end_y_top) / 20.0;

    for y in 0..20 {
        line(
            color,
            1.0,
            [
                gui_start_x,
                gui_start_y_top + delta_y_1 * (y as f64),
                gui_end_x,
                gui_end_y_top + delta_y_2 * (y as f64),
            ],
            con.transform,
            g,
        );
    }
}
