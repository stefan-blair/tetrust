use crate::drivers::Driver;
use crate::drivers::BoardTransition;
use crate::ui::rendering::*;
use crate::ui::utils::board_transition_progress::BoardTransitionsProgress;


#[derive(Clone, Copy)]
pub struct WidgetState<'a> {
    pub driver: &'a dyn Driver,
    pub transition: &'a BoardTransition,
    pub transition_progress: BoardTransitionsProgress
}

pub trait Widget {
    fn draw<'a>(&mut self, state: WidgetState, renderer: Renderer);
}
