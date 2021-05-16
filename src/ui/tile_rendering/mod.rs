use crate::game_core::utils::point::*;
use crate::ui::game_widgets::widget::WidgetState;

pub mod basic_renderer;


pub trait TileRenderer {
    fn render_tile(&mut self, tile: Point, pixel: Point, cell_size: i32);
}

pub trait TileRenderManager<'a> {
    type R: TileRenderer;

    fn get_rendering_state(&mut self, widget_state: WidgetState<'a>) -> Self::R;
}