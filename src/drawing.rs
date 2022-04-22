use std::f64::consts::PI;

use piston_window::line;
use piston_window::rectangle;
use piston_window::types::Color;
use piston_window::Context;
use piston_window::G2d;

use crate::player::Player;

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
        1.0,
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
    // tan ALFA = proti od. Y / prileh od. X
    let mut angle_point_start = 0.0;
    if player.x - start_x != 0.0 {
        angle_point_start = ((player.y - start_y) / (player.x - start_x)).atan();
        if angle_point_start > PI {
            angle_point_start -= 2.0 * PI;
        }
        if angle_point_start < PI {
            angle_point_start += 2.0 * PI;
        }
    }
    let mut angle_point_end = 0.0;
    if player.x - end_x != 0.0 {
        angle_point_end = ((player.y - end_y) / (player.x - end_x)).atan();
        if angle_point_end > PI {
            angle_point_end -= 2.0 * PI;
        }
        if angle_point_end < PI {
            angle_point_end += 2.0 * PI;
        }
    }
    let angle_diff_start = player.view_angle - angle_point_start;
    let angle_diff_end = player.view_angle - angle_point_end;

    /*if angle_diff_start.abs() > PI / 2.0 && angle_diff_end.abs() > PI / 2.0 {
        return;
    }*/

    let distance_from_player_start = MINIMAP_HEIGHT as f64
        / ((player.x - start_x).powf(2.0) + (player.y - start_y).powf(2.0)).sqrt()
        * 50.0;
    let distance_from_player_end = MINIMAP_HEIGHT as f64
        / ((player.x - end_x).powf(2.0) + (player.y - end_y).powf(2.0)).sqrt()
        * 50.0;

    let gui_start_x = to_gui_coord(500.0 - angle_diff_start * 100.0);
    let gui_start_y = to_gui_coord(distance_from_player_start + MINIMAP_HEIGHT as f64); // lenght from player
    let gui_end_x = to_gui_coord(500.0 - angle_diff_end * 100.0);
    let gui_end_y = to_gui_coord(distance_from_player_end + MINIMAP_HEIGHT as f64);

    line(
        color,
        1.0,
        [gui_start_x, gui_start_y, gui_end_x, gui_end_y],
        con.transform,
        g,
    );

    let gui_start_y = to_gui_coord(-distance_from_player_start + MINIMAP_HEIGHT as f64); // lenght from player
    let gui_end_y = to_gui_coord(-distance_from_player_end + MINIMAP_HEIGHT as f64);
    line(
        color,
        1.0,
        [gui_start_x, gui_start_y, gui_end_x, gui_end_y],
        con.transform,
        g,
    );
}
