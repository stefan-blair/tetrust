use rand::seq::SliceRandom;
use rand::{SeedableRng, rngs::StdRng};

use crate::game_core::tetriminos::*;


pub struct TetriminoChooser {
    current_bucket: Vec<(usize, &'static TetriminoType)>,
    tetrimino_types: &'static [TetriminoType],
    seeded_rng: StdRng,
}

impl TetriminoChooser {
    pub fn new(tetrimino_types: &'static [TetriminoType]) -> Self {
        Self {
            current_bucket: Vec::new(),
            tetrimino_types,
            seeded_rng: SeedableRng::from_seed([0; 32]),
        }
    }

    pub fn set_seed(&mut self, seed: Vec<u8>) {
        let mut seed_array: [u8; 32] = [0; 32];
        for (i, b) in seed.iter().cloned().enumerate() {
            if i >= 32 {
                break;
            }

            seed_array[i] = b;
        }

        self.seeded_rng = SeedableRng::from_seed(seed_array);
    }

    pub fn get_tetrimino_types(&self) -> &'static [TetriminoType] {
        self.tetrimino_types
    }

    pub fn choose_tetrimino_type(&mut self) -> (usize, &'static TetriminoType) {
        if self.current_bucket.is_empty() {
            self.current_bucket = self.tetrimino_types.iter().enumerate().collect();
            self.current_bucket.shuffle(&mut self.seeded_rng);
        }

        self.current_bucket.pop().unwrap()
    }
}
