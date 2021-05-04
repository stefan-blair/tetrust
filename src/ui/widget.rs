use crate::game_core::GameCore;
use crate::game_core::utils::point::Point;


pub trait Widget {
    fn draw(&self, engine: &GameCore, bounding_box: (Point, Point));
}