use macroquad::prelude::*;

use super::*;
use crate::game_core::GameCore;
use crate::drivers::BoardTransition;


pub struct BasicRenderManager;

impl<'a> TileRenderManager<'a> for BasicRenderManager {
    type R = BasicRenderer<'a>;

    fn get_rendering_state(&mut self, widget_state: WidgetState<'a>) -> Self::R {
        let transition_completion = (widget_state.transition_elapsed as f32) / (widget_state.transition_duration as f32);        
        BasicRenderer::new(widget_state.driver.get_game_core(), widget_state.transition, 1.0 - transition_completion)
    }
}

pub struct BasicRenderer<'a> {
    game_core: &'a GameCore,
    active_tetrimino_points: Vec<Point>,
    transition: &'a BoardTransition,
    alpha: f32,
}

impl<'a> BasicRenderer<'a> {
    fn new(game_core: &'a GameCore, transition: &'a BoardTransition, alpha: f32) -> Self {
        Self {
            game_core,
            active_tetrimino_points: game_core.get_active_tetrimino().get_points(),
            transition,
            alpha
        }
    }
}

impl<'a> TileRenderer for BasicRenderer<'a> {
    fn render_tile(&mut self, point: Point, pixel: Point, cell_size: i32) {
        let boarder = cell_size as f32 / 60.0;
        draw_rectangle(
            pixel.x() as f32 + boarder,
            pixel.y() as f32 + boarder,
            cell_size as f32 - 2.0 * boarder,
            cell_size as f32 - 2.0 * boarder,
            BLACK,
        );

        let mut alpha = 1.0;
        let mut active_tile_value = None;
        if self.game_core.get_board().is_point_filled(point) {
            active_tile_value = self
                .game_core
                .get_board()
                .get_cell(point);
            if self.transition
                .get_rows_deleted()
                .map_or(false, |rows| rows
                    .iter()
                    .find(|&&y| y == point.y())
                    .is_some()) {
                alpha = self.alpha
            } else if self.transition
                .get_points_deleted()
                .map_or(false, |points| points
                    .iter()
                    .find(|&&p| p == point)
                    .is_some()) {
                alpha = self.alpha
            }
        } else if let Some((i, _)) = self.active_tetrimino_points
            .iter()
            .enumerate()
            .find(|(_, p)| **p == point)
        {
            active_tile_value = self.game_core
                .get_active_tetrimino()
                .get_tetrimino()
                .values
                .get(i)
                .cloned();
        } else if self.game_core.get_ghost_tetriminio()
            .iter()
            .find(|p| **p == point)
            .is_some()
        {
            draw_rectangle(
                pixel.x() as f32 + boarder,
                pixel.y() as f32 + boarder,
                cell_size as f32 - 2.0 * boarder,
                cell_size as f32 - 2.0 * boarder,
                GRAY,
            );
            draw_rectangle_lines(
                pixel.x() as f32,
                pixel.y() as f32,
                cell_size as f32,
                cell_size as f32,
                boarder * 4.0,
                WHITE,
            );
        }

        if let Some(value) = active_tile_value {
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
                pixel.x() as f32 + boarder,
                pixel.y() as f32 + boarder,
                cell_size as f32 - 2.0 * boarder,
                cell_size as f32 - 2.0 * boarder,
                color,
            );    
        }
    }
}