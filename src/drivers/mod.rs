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
    fn next_frame(&mut self) -> Vec<BoardTransition>;

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

    fn fall_default(&mut self) -> (bool, Vec<BoardTransition>) {
        let game_core = self.get_game_core_mut();
        let (added, rows) = game_core.fall();
        if let Some(rows) = rows {
            if !rows.is_empty() {
                return (true, vec![BoardTransition::RowsDeleted(rows)]);
            }
        }
        (added, Vec::new())
    }

    fn fall(&mut self) -> Vec<BoardTransition> {
        self.fall_default().1
    }

    fn fastfall_default(&mut self) -> (i32, Vec<BoardTransition>) {
        let game_core = self.get_game_core_mut();
        let (translation, rows) = game_core.fastfall();
        if let Some(rows) = rows {
            if !rows.is_empty() {
                return (translation, vec![BoardTransition::RowsDeleted(rows)])
            }
        }
        (translation, vec![])
    }

    fn fastfall(&mut self) -> Vec<BoardTransition> {
        self.fastfall_default().1
    }

    fn rows_cleared(&mut self, _: Vec<i32>) -> Vec<BoardTransition> {
        Vec::new()
    }

    fn points_cleared(&mut self, mut points: Vec<Point>) -> Vec<BoardTransition> {
        // sort the points first by x, then by y
        points.sort_by_key(|p| (p.x(), p.y()));
        Vec::new()
    }

    fn finish_transition(&mut self, transition: BoardTransition) -> Vec<BoardTransition>{
        let board = self.get_game_core_mut().get_board_mut();
        match transition {
            BoardTransition::PointsDeleted(points) => {
                board.clear_points(&points);
                self.points_cleared(points)
            }
            BoardTransition::PointsFalling(points) => {
                let rows = board.translate_falling_points(points);
                println!("falling points cleared {:?}", rows);
                if rows.is_empty() {
                    Vec::new()
                } else {
                    vec![BoardTransition::RowsDeleted(rows)]
                }
            }
            BoardTransition::RowsDeleted(rows) => {
                board.clear_rows(rows.clone());
                self.rows_cleared(rows)
            }
        }
    }
}