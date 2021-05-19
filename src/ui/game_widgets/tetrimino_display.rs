use macroquad::prelude::*;

use crate::game_core::tetriminos::Tetrimino;
use crate::game_core::utils::point::Point;
use crate::game_core::GameCore;
use super::tiles;
use super::widget::*;

pub struct TetriminoDisplay {
    area: (Point, Point),
    dimensions: Point,
    extract_tetrimino: for<'a> fn(&'a GameCore) -> Option<&'a Tetrimino>,
}

impl TetriminoDisplay {
    pub fn new(
        area: (Point, Point),
        game_core: &GameCore,
        extract_tetrimino: for<'a> fn(&'a GameCore) -> Option<&'a Tetrimino>,
    ) -> Self {
        let width = game_core
            .get_tetrimino_types()
            .iter()
            .map(|t| t.get_dimensions().x())
            .max()
            .unwrap();
        let height = game_core
            .get_tetrimino_types()
            .iter()
            .map(|t| t.get_dimensions().y())
            .max()
            .unwrap();

        Self {
            area,
            dimensions: Point(width, height),
            extract_tetrimino,
        }
    }
}

impl Widget for TetriminoDisplay {
    fn draw(&mut self, state: WidgetState) {
        let area = self.area;
        let driver = state.driver;
        let game_core = driver.get_game_core();
        let dimensions = area.1 - area.0;

        draw_rectangle(
            area.0.x() as f32,
            area.0.y() as f32,
            dimensions.x() as f32,
            dimensions.y() as f32,
            GRAY,
        );

        if let Some(tetrimino) = (self.extract_tetrimino)(game_core) {
            let tetrimino_type = tetrimino.tetrimino_type;
            let points = tetrimino_type.get_points();
            let leftmost = points.iter().map(|p| p.x()).min().unwrap();
            let lowest = points.iter().map(|p| p.y()).min().unwrap();

            let points = points
                .into_iter()
                .map(|p| p - Point(leftmost, lowest))
                .collect::<Vec<_>>();

            let cell_size = (dimensions / self.dimensions).min(); 
            let length = self.dimensions.max();

            let padding = (Point::diag(length) - tetrimino_type.get_dimensions()) * Point::diag(cell_size / 2);

            for (i, point) in points.into_iter().enumerate() {
                // the point on the screen
                let pixel = Point(
                    point.x() * cell_size + area.0.x() + padding.x(),
                    area.1.y() - (point.y() + 1) * cell_size - padding.y(),
                );
                tiles::draw_active_tile(pixel, cell_size, tetrimino.values[i], 1.0)
            }
        }
    }
}
