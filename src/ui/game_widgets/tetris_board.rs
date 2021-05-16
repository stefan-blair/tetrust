use macroquad::prelude::*;

use crate::game_core::utils::point::Point;
use super::widget::{Widget, WidgetState};
use crate::ui::tile_rendering::*;
use crate::ui::tile_rendering::basic_renderer::*;

pub struct TetrisBoard {
    area: (Point, Point)
}

impl TetrisBoard {
    pub fn new(area: (Point, Point)) -> Self {
        Self {
            area
        }
    }
}

impl Widget for TetrisBoard {
    fn draw(&self, state: WidgetState) {
        let mut render_manager = BasicRenderManager;

        let driver = state.driver;
        let transition = state.transition;
        let transition_completion = (state.transition_elapsed as f32) / (state.transition_duration as f32);
        let area = self.area;
        let game_core = driver.get_game_core();
        let dimensions = area.1 - area.0;

        draw_rectangle(
            area.0.x() as f32,
            area.0.y() as f32,
            dimensions.x() as f32,
            dimensions.y() as f32,
            WHITE,
        );

        let cell_size = std::cmp::min(
            (area.1.x() - area.0.x()) / game_core.get_board().get_width() as i32,
            (area.1.y() - area.0.y()) / game_core.get_board().get_height() as i32,
        );

        // counts the number of rows under the current row that are dissapearing
        let mut deleted_rows = 0;
        // draw the tiles
        for y in 0..game_core.get_board().get_height() as i32 {
            for x in 0..game_core.get_board().get_width() as i32 {
                // the point on the board
                let point = Point(x, y);
 
                let point_fall = if let Some(points) = transition.get_points_falling() {
                    points
                        .iter()
                        .find(|(p, _)| *p == point)
                        .map(|(_, f)| *f)
                        .unwrap_or(0)
                } else {
                    deleted_rows
                };

                // using the number of rows beneath the current row that are disappearing, calculate fall based on the elapsed frames of the animation
                let point_fall_offset = (cell_size * point_fall) as f32 * transition_completion;
                let point_fall_offset = Point::unit_y(point_fall_offset as i32);
                // the point on the screen
                let pixel = Point(
                    x * cell_size + area.0.x(),
                    area.0.y() + (game_core.get_board().get_height() as i32 - y - 1) * cell_size,
                ) + point_fall_offset;

                render_manager.get_rendering_state(state).render_tile(point, pixel, cell_size);
            }

            if let Some(rows) = transition.get_rows_deleted() {
                if rows.contains(&y) {
                    deleted_rows += 1;
                }
            }
        }
    }
}