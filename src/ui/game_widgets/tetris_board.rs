use macroquad::prelude::*;

use crate::game_core::utils::point::Point;
use super::widget::{Widget, WidgetState};
use crate::ui::rendering::*;


pub struct TetrisBoard {
    area: (Point, Point),
}

impl TetrisBoard{
    pub fn new(area: (Point, Point)) -> Self {
        Self {
            area,
        }
    }
}

impl Widget for TetrisBoard {
    fn draw<'a>(&mut self, state: WidgetState, mut rendering_state: Box<dyn TileRenderer + 'a>) {
        let driver = state.driver;
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

                rendering_state.render_tile_at(point, pixel, cell_size);
            }
        }
    }
}