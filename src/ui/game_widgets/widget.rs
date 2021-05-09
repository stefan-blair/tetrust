use crate::drivers::Driver;
use crate::drivers::BoardTransition;


#[derive(Clone, Copy)]
pub struct WidgetState<'a, 'b> {
    pub driver: &'a dyn Driver<'b>,
    pub transition: Option<&'a BoardTransition>,
    pub transition_elapsed: usize,
    pub transition_duration: usize
}

pub trait Widget {
    fn draw(&self, state: WidgetState);
}
