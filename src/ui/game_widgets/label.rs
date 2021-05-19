use macroquad::prelude::*;

use super::widget::*;
use crate::drivers::Driver;
use crate::game_core::utils::point::Point;


pub struct Label {
    location: Point,
    color: Color,
    font_size: f32,
    extract_string: fn(&dyn Driver) -> String,
}

impl Label {
    pub fn new(
        location: Point, 
        color: Color, 
        font_size: f32,
        extract_string: fn(&dyn Driver) -> String
    ) -> Self {
        Self {
            location,
            color,
            font_size,
            extract_string
        }
    }
}

impl Widget for Label {
    fn draw(&mut self, state: WidgetState) {
        let text = (self.extract_string)(state.driver);
        draw_text(&text, self.location.x() as f32, self.location.y() as f32, self.font_size, self.color);
    }
}