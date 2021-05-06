use macroquad::prelude::*;

use crate::drivers::Driver;
use crate::game_core::utils::point::Point;
use crate::ui::tiles;
use crate::ui::widget::Widget;

pub struct TetrisBoard;

impl Widget for TetrisBoard {
    fn draw(&self, driver: &Driver, area: (Point, Point)) {
        let game_core = driver.get_game_core();
        let dimensions = area.1 - area.0;

        draw_rectangle(
            area.0.x() as f32,
            area.0.y() as f32,
            dimensions.x() as f32,
            dimensions.y() as f32,
            BLACK,
        );

        let cell_size = std::cmp::min(
            (area.1.x() - area.0.x()) / game_core.get_board().get_width() as i32,
            (area.1.y() - area.0.y()) / game_core.get_board().get_height() as i32,
        );

        let active_tetrimino_points = game_core.get_active_tetrimino().get_points();
        let ghost_tetrimino_points = game_core.get_ghost_tetriminio().get_points();
        // draw the tiles
        for y in 0..game_core.get_board().get_height() as i32 {
            for x in 0..game_core.get_board().get_width() as i32 {
                // the point on the board
                let point = Point(x, y);
                // the point on the screen
                let pixel = Point(
                    x * cell_size + area.0.x(),
                    area.0.y() + (game_core.get_board().get_height() as i32 - y - 1) * cell_size,
                );

                if game_core.get_board().is_point_filled(point) {
                    tiles::draw_active_tile(pixel, cell_size)
                } else if active_tetrimino_points
                    .iter()
                    .find(|p| **p == point)
                    .is_some()
                {
                    tiles::draw_filled_tile(pixel, cell_size)
                } else if ghost_tetrimino_points
                    .iter()
                    .find(|p| **p == point)
                    .is_some()
                {
                    tiles::draw_ghost_tile(pixel, cell_size)
                } else {
                    tiles::draw_empty_tile(pixel, cell_size)
                }

                if point == game_core.get_board().get_spawn_point() {
                    tiles::draw_debug_tile(pixel, cell_size / 4)
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
