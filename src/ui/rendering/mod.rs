use crate::game_core::utils::point::*;
use crate::ui::game_widgets::widget::WidgetState;

pub mod basic_renderer;
pub mod basic_tileset_renderer;


pub trait TileRenderer {
    /**
     * This will be called on each piece on the board starting from (0, 0) to
     * (width, height), going by row.
     */
    fn render_tile_at(&mut self, tile: Point, pixel: Point, cell_size: i32);

    /**
     * This function is used to render a piece anywhere on screen with a given
     * value.
     */
    fn render_tile(&mut self, pixel: Point, cell_size: i32, value: u32, alpha: f32);
}

pub trait TileRenderManager {
    fn get_rendering_state<'a>(&'a mut self, widget_state: WidgetState<'a>) -> Box<dyn TileRenderer + 'a>;
}