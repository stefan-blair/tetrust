use crate::game_core::GameCore;
use crate::game_core::utils::point::Point;
use crate::game_core::utils::orientations::Direction;

pub mod base_driver;
pub mod sticky_driver;


#[derive(Debug)]
pub enum BoardTransition {
    PointsDeleted(Vec<Point>),
    RowsDeleted(Vec<i32>),
    PointsFalling(Vec<(Point, i32)>),
}

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

    fn fall(&mut self) -> Vec<BoardTransition> {
        let game_core = self.get_game_core_mut();
        if let (_, Some(rows)) = game_core.fall() {
            if !rows.is_empty() {
                return vec![BoardTransition::RowsDeleted(rows)]
            }
        }
        Vec::new()
    }

    fn fastfall(&mut self) -> Vec<BoardTransition> {
        let game_core = self.get_game_core_mut();
        if let Some(rows) = game_core.fastfall() {
            if !rows.is_empty() {
                return vec![BoardTransition::RowsDeleted(rows)]
            }
        }
        Vec::new()
    }

    fn finish_transition(&mut self, transition: BoardTransition) {
        let game_core = self.get_game_core_mut();
        match transition {
            BoardTransition::PointsDeleted(points) => {
                game_core.clear_points(points)
            }
            BoardTransition::PointsFalling(points) => {
                game_core.translate_falling_points(points)
            }
            BoardTransition::RowsDeleted(rows) => {
                game_core.clear_rows(rows)
            }
        }
    }
}