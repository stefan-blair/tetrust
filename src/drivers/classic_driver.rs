use crate::drivers::*;
use crate::game_core::GameCore;


pub struct ClassicDriver {
    last_clear_was_tetris: bool,
    default_driver: DefaultDriver,
}

impl Default for ClassicDriver {
    fn default() -> Self {
        Self {
            last_clear_was_tetris: false,
            default_driver: DefaultDriverBuilder::new().build()
        }
    }
}

impl ClassicDriver {
    pub fn _new(core: GameCore, get_gravity: fn(usize) -> f32, lock_delay: usize) -> Self {
        Self {
            last_clear_was_tetris: false,
            default_driver: DefaultDriver::new(core, get_gravity, lock_delay)
        }
    }

    fn update_score(&mut self, increment: usize) {
        self.default_driver.score += increment;
        let level = self.default_driver.score / 5;
        if self.default_driver.level < level && level < 15 {
            self.default_driver.level = level;
        }
    }
}

impl Driver for ClassicDriver {
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
        let score_update = match rows.len() {
            1 => 1,
            2 => 3,
            3 => 5,
            4 => if self.last_clear_was_tetris {
                    12
                } else {
                    self.last_clear_was_tetris = true;
                    8
                }
            _ => 0
        };

        self.last_clear_was_tetris = false;
        self.update_score(score_update);
        self.default_driver.rows_cleared(rows)
    }

    fn points_cleared(&mut self, points: Vec<Point>) -> BoardTransition {
        self.default_driver.points_cleared(points)
    }

    fn points_fell(&mut self, points: Vec<(Point, i32)>, full_rows: Vec<i32>) -> BoardTransition {
        self.default_driver.points_fell(points, full_rows)
    }
}
