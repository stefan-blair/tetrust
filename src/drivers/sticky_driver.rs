use rand::thread_rng;
use rand::Rng;

use crate::drivers::*;
use crate::game_core::GameCore;
use crate::game_core::utils::point::Point;
use super::utils::recursive_physics::*;
use super::utils::tetrimino_chooser::TetriminoChooser;


pub struct StickyGenerator {
    tetrimino_chooser: TetriminoChooser,
}

impl StickyGenerator {
    pub fn new(tetrimino_types: &'static [TetriminoType]) -> Box<Self> {
        Box::new(Self {
            tetrimino_chooser: TetriminoChooser::new(tetrimino_types),
        })
    }
}

impl TetriminoGenerator for StickyGenerator {
    fn next(&mut self) -> Tetrimino {
        let (index, tetrimino_type) = self.tetrimino_chooser.choose_tetrimino_type();
        let mut values = vec![index as u32; 4];

        let make_multicolored = thread_rng().gen::<usize>() % 2;
        if make_multicolored == 0 {
            let idx_1 = thread_rng().gen::<usize>() % values.len();
            let idx_2 = thread_rng().gen::<usize>() % values.len();

            let new_value = (index as u32 + 1) % values.len() as u32;
            values[idx_1] = new_value;
            values[idx_2] = new_value
        }

        tetrimino_type.instance(values)
    }
}

pub struct StickyDriver {
    default_driver: DefaultDriver
}

impl Default for StickyDriver {
    fn default() -> Self {
        Self {
            default_driver: DefaultDriverBuilder::new().build()
        }
    }
}

impl StickyDriver {
    pub fn _new(core: GameCore, get_gravity: fn(usize) -> f32, lock_delay: usize) -> Self {
        Self {
            default_driver: DefaultDriver::new(core, get_gravity, lock_delay)
        }
    }
}

impl Driver for StickyDriver {
    fn get_generator(tetrimino_types: &'static [TetriminoType]) -> Box<dyn TetriminoGenerator> {
        StickyGenerator::new(tetrimino_types)
    }

    fn get_game_core(&self) -> &GameCore {
        self.default_driver.get_game_core()
    }

    fn get_game_core_mut(&mut self) -> &mut GameCore {
        self.default_driver.get_game_core_mut()
    }

    fn get_score(&self) -> usize {
        self.default_driver.get_score()
    }

    fn get_level(&self) -> usize {
        self.default_driver.get_level()
    }

    fn next_frame(&mut self) -> BoardTransition {
        self.default_driver.next_frame()
    }

    fn hold(&mut self) {
        self.default_driver.hold()
    }

    fn fall(&mut self) -> (bool, BoardTransition) {
        let points = self.get_game_core().get_active_tetrimino().get_points();
        let (added, mut transitions) = self.default_driver.fall();
        if added {
            // calculate falling points
            let falls = calculate_sticky_falls(self.get_game_core().get_board(), points);
            if !falls.is_empty() {
                transitions.add_to_transition(BoardTransition::new().with_points_falling(falls))
            }
        }

        (added, transitions)
    }

    fn fastfall(&mut self) -> (i32, BoardTransition) {
        let mut points = self.get_game_core().get_active_tetrimino().get_points();
        let (translation, mut transitions) = self.default_driver.fastfall();
        points = points.into_iter().map(|p| p - Point::unit_y(translation)).collect::<Vec<_>>();
        let falls = calculate_sticky_falls(self.get_game_core().get_board(), points);
        if !falls.is_empty() {
            transitions.add_to_transition(BoardTransition::new().with_points_falling(falls));
        }
        (translation, transitions)
    }

    fn rows_cleared(&mut self, rows: Vec<i32>) -> BoardTransition {
        let falls = calculate_sticky_falls_from_rows(self.get_game_core().get_board(), rows);
        if !falls.is_empty() {
            BoardTransition::new().with_points_falling(falls)
        } else {
            BoardTransition::new()
        }
    }

    fn points_cleared(&mut self, points: Vec<Point>) -> BoardTransition {
        self.default_driver.points_cleared(points)
    }

    fn points_fell(&mut self, points: Vec<(Point, i32)>, full_rows: Vec<i32>) -> BoardTransition {
        self.default_driver.points_fell(points, full_rows)
    }
}