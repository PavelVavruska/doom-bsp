use piston_window::line;
use piston_window::rectangle;
use piston_window::types::Color;
use piston_window::Context;
use piston_window::G2d;

const BLOCK_SIZE: f64 = 1.0;

pub fn to_gui_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_gui_coord_u32(game_coord: i32) -> u32 {
    to_gui_coord(game_coord) as u32
}

pub fn draw_rectange(
    color: Color,
    start_x: i32,
    start_y: i32,
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

pub fn draw_line(
    color: Color,
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
    con: &Context,
    g: &mut G2d,
) {
    let gui_start_x = to_gui_coord(start_x);
    let gui_start_y = to_gui_coord(start_y);
    let gui_end_x = to_gui_coord(end_x);
    let gui_end_y = to_gui_coord(end_y);

    line(
        color,
        10.0,
        [gui_start_x, gui_start_y, gui_end_x, gui_end_y],
        con.transform,
        g,
    );
}
