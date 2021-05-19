use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng, rngs::StdRng, RngCore};

use crate::game_core::tetriminos::*;


pub struct TetriminoChooser {
    current_bucket: Vec<(usize, &'static TetriminoType)>,
    tetrimino_types: &'static [TetriminoType],
    seeded_rng: Option<StdRng>,
    seed: Option<Vec<u8>>
}

impl TetriminoChooser {
    pub fn new(tetrimino_types: &'static [TetriminoType]) -> Self {
        Self {
            current_bucket: Vec::new(),
            tetrimino_types,
            seeded_rng: None,
            seed: None
        }
    }

    pub fn with_seed(mut self, seed: Vec<u8>) -> Self {
        let mut seed_array: [u8; 32] = [0; 32];
        for (i, b) in seed.iter().cloned().enumerate() {
            if i >= 32 {
                break;
            }

            seed_array[i] = b;
        }

        self.seeded_rng = Some(SeedableRng::from_seed(seed_array));
        self.seed = Some(seed);

        self
    }

    pub fn get_seed(&self) -> Option<&Vec<u8>> {
        self.seed.as_ref()
    }

    pub fn choose_tetrimino_type(&mut self) -> (usize, &'static TetriminoType) {
        let trng = &mut thread_rng();

        // if there is a specific seeded rng, use that, otherwise use the thread rng
        let rng: &mut dyn RngCore = if let Some(rng) = self.seeded_rng.as_mut() {
            println!("using seeded");
            rng
        } else {
            trng
        };

        if self.current_bucket.is_empty() {
            self.current_bucket = self.tetrimino_types.iter().enumerate().collect();
            self.current_bucket.shuffle(rng);
            println!("current bucket: {:?}", self.current_bucket.iter().map(|(i, _)| i).collect::<Vec<_>>());
        }

        self.current_bucket.pop().unwrap()
    }
}

#[cfg(test)]
mod tetrimino_chooser_seeded_random_tests {
    use super::*;

    #[test]
    fn test_seeded_random() {
        let seed_array: [u8; 32] = [0; 32];
        let mut seeded_rng: StdRng = SeedableRng::from_seed(seed_array);

        for _ in 0..10 {
            println!("num: {:?}", seeded_rng.gen::<usize>());
        }

        assert!(false);
    }
}