use crate::drivers::Driver;
use crate::drivers::BoardTransition;


#[derive(Clone, Copy)]
pub struct WidgetState<'a> {
    pub driver: &'a dyn Driver,
    pub transition: &'a BoardTransition,
    pub transition_elapsed: usize,
    pub transition_duration: usize
}

pub trait Widget {
    fn draw(&mut self, state: WidgetState);
}
