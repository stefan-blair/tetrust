use macroquad::prelude::*;

use crate::game_core::utils::point::Point;

pub fn draw_active_tile(point: Point, cell_size: i32, value: u32, alpha: f32) {
    let boarder = cell_size as f32 / 40.0;
    let mut color = match value {
        1 => RED,
        2 => BLUE,
        3 => GREEN,
        4 => YELLOW,
        5 => ORANGE,
        6 => MAGENTA,
        7 => SKYBLUE,
        _ => GOLD,
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

pub fn draw_empty_tile(point: Point, cell_size: i32) {
    let boarder = cell_size as f32 / 20.0;
    draw_rectangle(
        point.x() as f32 + boarder,
        point.y() as f32 + boarder,
        cell_size as f32 - 2.0 * boarder,
        cell_size as f32 - 2.0 * boarder,
        GRAY,
    );
}

pub fn draw_ghost_tile(point: Point, cell_size: i32) {
    let boarder = cell_size as f32 / 20.0;
    draw_rectangle(
        point.x() as f32 + boarder,
        point.y() as f32 + boarder,
        cell_size as f32 - 2.0 * boarder,
        cell_size as f32 - 2.0 * boarder,
        GRAY,
    );
    draw_rectangle_lines(
        point.x() as f32,
        point.y() as f32,
        cell_size as f32,
        cell_size as f32,
        boarder * 4.0,
        WHITE,
    );
}
