use macroquad::prelude::*;

use crate::game_core::utils::point::Point;

pub fn draw_active_tile(point: Point, cell_size: i32) {
    let boarder = cell_size as f32 / 40.0;
    draw_rectangle(
        point.x() as f32 + boarder,
        point.y() as f32 + boarder,
        cell_size as f32 - 2.0 * boarder,
        cell_size as f32 - 2.0 * boarder,
        RED,
    );
}

pub fn draw_filled_tile(point: Point, cell_size: i32) {
    let boarder = cell_size as f32 / 40.0;
    draw_rectangle(
        point.x() as f32 + boarder,
        point.y() as f32 + boarder,
        cell_size as f32 - 2.0 * boarder,
        cell_size as f32 - 2.0 * boarder,
        GREEN,
    );
}

pub fn draw_debug_tile(point: Point, cell_size: i32) {
    draw_rectangle(
        point.x() as f32,
        point.y() as f32,
        cell_size as f32,
        cell_size as f32,
        BLUE,
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
