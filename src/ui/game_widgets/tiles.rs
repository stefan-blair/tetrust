use macroquad::prelude::*;

use crate::game_core::utils::point::Point;

pub fn draw_active_tile(point: Point, cell_size: i32, value: u32, alpha: f32) {
    let boarder = cell_size as f32 / 60.0;
    let mut color = match value {
        0 => RED,
        1 => BLUE,
        2 => GREEN,
        3 => YELLOW,
        4 => ORANGE,
        5 => MAGENTA,
        6 => SKYBLUE,
        _ => LIGHTGRAY,
    };

    color.a = alpha;

    draw_rectangle(
        point.x() as f32 + boarder,
        point.y() as f32 + boarder,
        cell_size as f32 - 2.0 * boarder,
        cell_size as f32 - 2.0 * boarder,
        color,
    );
}