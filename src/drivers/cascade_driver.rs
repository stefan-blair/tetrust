use crate::drivers::*;
use crate::game_core::GameCore;
use crate::game_core::utils::point::Point;
use super::utils::recursive_physics::calculate_sticky_falls_from_rows;
use super::utils::tetrimino_chooser::TetriminoChooser;


pub struct CascadeGenerator {
    tetrimino_chooser: TetriminoChooser,
    current_index: usize,
    num_tetriminos: usize,
}

impl CascadeGenerator {
    pub fn new(tetrimino_types: &'static [TetriminoType]) -> Box<Self> {
        Box::new(Self {
            tetrimino_chooser: TetriminoChooser::new(tetrimino_types),
            current_index: 0,
            num_tetriminos: tetrimino_types.len()
        })
    }
}

impl TetriminoGenerator for CascadeGenerator {
    fn next(&mut self) -> Tetrimino {
        let (index, tetrimino_type) = self.tetrimino_chooser.choose_tetrimino_type();
        let values = vec![(index + (self.current_index * self.num_tetriminos)) as u32; 4];
        self.current_index += 1;
        tetrimino_type.instance(values)
    }
}

pub struct CascadeDriver {
    default_driver: DefaultDriver
}

impl Default for CascadeDriver {
    fn default() -> Self {
        Self {
            default_driver: DefaultDriverBuilder::new().build()
        }
    }
}

impl CascadeDriver {
    pub fn _new(core: GameCore, get_gravity: fn(usize) -> f32, lock_delay: usize) -> Self {
        Self {
            default_driver: DefaultDriver::new(core, get_gravity, lock_delay)
        }
    }
}

impl Driver for CascadeDriver {
    fn get_generator(tetrimino_types: &'static [TetriminoType]) -> Box<dyn TetriminoGenerator> {
        CascadeGenerator::new(tetrimino_types)
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
        self.default_driver.fall()
    }

    fn fastfall(&mut self) -> (i32, BoardTransition) {
        self.default_driver.fastfall()
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