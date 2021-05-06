use crate::game_core::GameCore;
use crate::game_core::utils::point::Point;
use crate::game_core::utils::orientations::Direction;

pub mod base_driver;
pub mod sticky_driver;


pub trait Driver<'a> {
    fn get_game_core(&self) -> &GameCore<'a>;
    fn get_game_core_mut(&mut self) -> &mut GameCore<'a>;
    fn next_frame(&mut self);

    fn translate_left(&mut self) -> bool {
        self.get_game_core_mut().translate(Point(-1, 0))
    }

    fn translate_right(&mut self) -> bool {
        self.get_game_core_mut().translate(Point(1, 0))
    }

    fn rotate_clockwise(&mut self) -> bool {
        self.get_game_core_mut().rotate(Direction::Clockwise)
    }

    fn rotate_counterclockwise(&mut self) -> bool {
        self.get_game_core_mut().rotate(Direction::CounterClockwise)
    }

    fn fall(&mut self) {
        let game_core = self.get_game_core_mut();
        if let Some(rows) = game_core.fall() {
            game_core.clear_rows(rows)
        }        
    }

    fn fastfall(&mut self) {
        let game_core = self.get_game_core_mut();
        if let Some(rows) = game_core.fastfall() {
            game_core.clear_rows(rows)
        }
    }
}