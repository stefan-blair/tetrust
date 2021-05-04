use macroquad::prelude::*;

use crate::game_core::GameCore;
use crate::game_core::utils::point::Point;
use crate::ui::widget::Widget;


pub struct TetrisBoard;

fn draw_active_tile(point: Point, cell_size: i32) {
    draw_rectangle(point.x() as f32 + 0.5, point.y() as f32 + 0.5, cell_size as f32 - 1.0, cell_size as f32 - 1.0, RED);
}

fn draw_filled_tile(point: Point, cell_size: i32) {
    draw_rectangle(point.x() as f32 + 0.5, point.y() as f32 + 0.5, cell_size as f32 - 1.0, cell_size as f32 - 1.0, GREEN);
}

fn draw_debug_tile(point: Point, cell_size: i32) {
    draw_rectangle(point.x() as f32, point.y() as f32, cell_size as f32, cell_size as f32, BLUE);
}

fn draw_empty_tile(point: Point, cell_size: i32) {
    draw_rectangle(point.x() as f32 + 1.0, point.y() as f32 + 1.0, cell_size as f32 - 2.0, cell_size as f32 - 2.0, GRAY);
}

fn draw_ghost_tile(point: Point, cell_size: i32) {
    draw_rectangle(point.x() as f32 + 1.0, point.y() as f32 + 1.0, cell_size as f32 - 2.0, cell_size as f32 - 2.0, GRAY);
    draw_rectangle_lines(point.x() as f32, point.y() as f32, cell_size as f32, cell_size as f32, 4.0, WHITE);
}

impl Widget for TetrisBoard {
    fn draw(&self, engine: &GameCore, area: (Point, Point)) {
        clear_background(BLUE);
        draw_rectangle(area.0.x() as f32, area.0.y() as f32, area.1.x() as f32, area.1.y() as f32, BLACK);

        let cell_size = std::cmp::min(
            (area.1.x() - area.0.x()) / engine.get_board().get_width() as i32,
            (area.1.y() - area.0.y()) / engine.get_board().get_height() as i32);

        let active_tetrimino_points = engine.get_active_tetrimino().get_points();
        let ghost_tetrimino_points = engine.get_ghost_tetriminio().get_points();
        // draw the tiles
        for y in 0..engine.get_board().get_height() as i32 {
            for x in 0..engine.get_board().get_width() as i32 {
                // the point on the board
                let point = Point(x, y);
                // the point on the screen
                let pixel = Point(x * cell_size + area.0.x(), area.0.y() + (engine.get_board().get_height() as i32 - y - 1) * cell_size);

                if engine.get_board().is_point_filled(point) {
                    draw_active_tile(pixel, cell_size)
                } else if active_tetrimino_points.iter().find(|p| **p == point).is_some() {
                    draw_filled_tile(pixel, cell_size)
                } else if ghost_tetrimino_points.iter().find(|p| **p == point).is_some() {
                    draw_ghost_tile(pixel, cell_size)
                } else {
                    draw_empty_tile(pixel, cell_size)
                }

                if point == engine.get_board().get_spawn_point() {
                    draw_debug_tile(pixel, cell_size / 4)
                }
            }
        }
    }
}


/*
        clear_background(RED);

        draw_shapes();

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

*/

// fn draw_shapes() {
//     draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
//     draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
//     draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
// }
