use macroquad::prelude::*;

use super::*;
use crate::game_core::GameCore;
use crate::drivers::BoardTransition;
use crate::ui::assets::tilemap::TileMap;


#[derive(Clone)]
pub struct BasicTilesetRenderManager {
    tile_map: TileMap
}

impl BasicTilesetRenderManager {
    pub async fn new(image: &str, info: &str) -> Self {
        Self {
            tile_map: TileMap::new(image, info).await
        }
    }
}

impl TileRenderManager for BasicTilesetRenderManager {
    fn get_rendering_state<'a>(&'a mut self, widget_state: WidgetState<'a>) -> Box<dyn TileRenderer + 'a> {
        let transition_completion = (widget_state.transition_elapsed as f32) / (widget_state.transition_duration as f32);        
        Box::new(
            BasicTilesetRenderer::new(
                widget_state.driver.get_game_core(), 
                widget_state.transition, 
                transition_completion,
                &self.tile_map))
    }
}

pub struct BasicTilesetRenderer<'a> {
    game_core: &'a GameCore,
    active_tetrimino_points: Vec<Point>,
    transition: &'a BoardTransition,
    transition_completion: f32,

    deleted_rows: i32,
    last_y: i32,

    tile_map: &'a TileMap
}

impl<'a> BasicTilesetRenderer<'a> {
    fn new(game_core: &'a GameCore, transition: &'a BoardTransition, transition_completion: f32, tile_map: &'a TileMap) -> Self {
        Self {
            game_core,
            active_tetrimino_points: game_core.get_active_tetrimino().get_points(),
            transition,
            transition_completion,

            deleted_rows: 0,
            last_y: 0,

            tile_map
        }
    }
}

impl<'a> TileRenderer for BasicTilesetRenderer<'a> {
    fn render_tile(&mut self, point: Point, pixel: Point, cell_size: i32) {
        // if the current tile is on a new row, update the rows deleted
        if point.y() > self.last_y {
            self.last_y = point.y();
            if let Some(rows) = self.transition.get_rows_deleted() {
                if rows.contains(&(point.y() - 1)) {
                    self.deleted_rows += 1;
                }
            }
        }

        let boarder = cell_size as f32 / 100.0;
        // draw an empty rectangle first, to fill the blank square
        draw_rectangle(
            pixel.x() as f32 + boarder,
            pixel.y() as f32 + boarder,
            cell_size as f32 - 2.0 * boarder,
            cell_size as f32 - 2.0 * boarder,
            BLACK,
        );

        let mut alpha = 1.0;
        // if the current tile is active it will contain a value
        let mut active_tile_value = None;
        // check if the current point is filled
        if self.game_core.get_board().is_point_filled(point) {
            active_tile_value = self
                .game_core
                .get_board()
                .get_cell(point);
            // if the point is filled, check if the point is being deleted in the current transition
            if self.transition
                .get_rows_deleted()
                .map_or(false, |rows| rows
                    .iter()
                    .find(|&&y| y == point.y())
                    .is_some()) {
                alpha = 1.0 - self.transition_completion;
            } else if self.transition
                .get_points_deleted()
                .map_or(false, |points| points
                    .iter()
                    .find(|&&p| p == point)
                    .is_some()) {
                alpha = 1.0 - self.transition_completion;
            }
        // check if the current active tetrimino is currently taking up the tile
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
        // finally, check if the ghost of the active tetrimino is taking up the tile
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
            let mut point_fall = self.deleted_rows;
            if let Some(points) = self.transition.get_points_falling() {
                point_fall += points
                    .iter()
                    .find(|(p, _)| *p == point)
                    .map(|(_, f)| *f)
                    .unwrap_or(0)
            }

            // using the number of rows beneath the current row that are disappearing, calculate fall based on the elapsed frames of the animation
            let point_fall_offset = (cell_size * point_fall) as f32 * self.transition_completion;
            let point_fall_offset = Point::unit_y(point_fall_offset as i32);
            let pixel = pixel + point_fall_offset;

            let rect = self.tile_map.tiles[value as usize % self.tile_map.tiles.len()];
            let dest_size = cell_size as f32;

            let mut color = WHITE;
            color.a = alpha;

            draw_texture_ex(
                self.tile_map.tileset,
                pixel.x() as f32,
                pixel.y() as f32,
                color,
                DrawTextureParams {
                    dest_size: Some(vec2(dest_size, dest_size)),
                    source: Some(Rect::new(
                        rect.x as f32,
                        rect.y as f32,
                        rect.dim as f32,
                        rect.dim as f32
                    )),
                    ..Default::default()
                },
            );
        }
    }
}