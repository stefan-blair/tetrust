use crate::drivers::Driver;
use crate::game_core::utils::point::Point;

pub trait Widget {
    fn draw(&self, engine: &Driver, bounding_box: (Point, Point));
}
