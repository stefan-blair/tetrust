use macroquad::prelude::*;

use crate::drivers::{Driver, BoardTransition};
use crate::game_core::utils::point::Point;
use crate::ui::tiles;
use crate::ui::widget::Widget;

pub struct TetrisBoard;

impl Widget for TetrisBoard {
    fn draw(&self, driver: &dyn Driver, area: (Point, Point), transition: Option<&BoardTransition>, transition_elapsed: usize, transition_total: usize) {
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

        // counts the number of rows under the current row that are dissapearing
        let mut disappearing_rows_under = 0;
        // draw the tiles
        for y in 0..game_core.get_board().get_height() as i32 {
            let mut alpha = 1.0;
            if let Some(BoardTransition::RowsDeleted(rows)) = transition {
                if rows.contains(&y) {
                    alpha = 1.0 - (transition_elapsed as f32 / transition_total as f32);
                }
            }

            for x in 0..game_core.get_board().get_width() as i32 {
                // the point on the board
                let point = Point(x, y);
                // using the number of rows beneath the current row that are disappearing, calculate fall based on the elapsed frames of the animation
                let disappearing_row_offset = (cell_size * disappearing_rows_under) as f32 * transition_elapsed as f32 / transition_total as f32;
                let disappearing_row_offset = Point::unit_y(disappearing_row_offset as i32);
                // the point on the screen
                let pixel = Point(
                    x * cell_size + area.0.x(),
                    area.0.y() + (game_core.get_board().get_height() as i32 - y - 1) * cell_size,
                );

                tiles::draw_empty_tile(pixel, cell_size);

                if game_core.get_board().is_point_filled(point) {
                    let value = game_core.get_board().get_cell(point).unwrap();
                    tiles::draw_active_tile(pixel + disappearing_row_offset, cell_size, value, alpha)
                } else if let Some((i, _)) = active_tetrimino_points
                    .iter()
                    .enumerate()
                    .find(|(_, p)| **p == point)
                {
                    tiles::draw_active_tile(
                        pixel, 
                        cell_size, 
                        game_core.get_active_tetrimino().get_tetrimino().get_values()[i],
                        1.0
                    )
                } else if ghost_tetrimino_points
                    .iter()
                    .find(|p| **p == point)
                    .is_some()
                {
                    tiles::draw_ghost_tile(pixel, cell_size)
                }
            }

            if let Some(BoardTransition::RowsDeleted(rows)) = transition {
                if rows.contains(&y) {
                    disappearing_rows_under += 1;
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
