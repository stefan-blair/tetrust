use crate::drivers::Driver;
use crate::drivers::BoardTransition;
use crate::game_core::utils::point::Point;

pub trait Widget {
    fn draw(&self, driver: &dyn Driver, area: (Point, Point), transition: Option<&BoardTransition>, transition_elapsed: usize, transition_total: usize);
}
