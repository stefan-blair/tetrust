use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::game_core::tetriminos::*;


pub struct TetriminoChooser {
    current_bucket: Vec<(usize, &'static TetriminoType)>,
    tetrimino_types: &'static [TetriminoType]
}

impl TetriminoChooser {
    pub fn new(tetrimino_types: &'static [TetriminoType]) -> Self {
        Self {
            current_bucket: Vec::new(),
            tetrimino_types
        }
    }

    pub fn choose_tetrimino_type(&mut self) -> (usize, &'static TetriminoType) {
        if self.current_bucket.is_empty() {
            self.current_bucket = self.tetrimino_types.iter().enumerate().collect();
            self.current_bucket.shuffle(&mut thread_rng());
        }

        self.current_bucket.pop().unwrap()
    }
}
