use rand::thread_rng;
use rand::Rng;

use crate::drivers::*;
use crate::game_core::utils::point::Point;
use crate::game_core::defaults;
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

    fn get_tetrimino_types(&self) -> &'static [TetriminoType] {
        self.tetrimino_chooser.get_tetrimino_types()
    }
    
    fn set_seed(&mut self, seed: Vec<u8>) {
        self.tetrimino_chooser.set_seed(seed);
    }
}

pub struct StickyDriver {
    driver_core: DriverCore
}

impl BuildableDriver for StickyDriver {
    type Data = ();

    fn initialize(builder: DriverBuilder<Self>) -> DriverBuilder<Self> {
        builder
            .with_tetrimino_generator(
                StickyGenerator::new(defaults::tetriminos::TETRIMINOS))
    }
 
    fn build(mut builder: DriverBuilder<Self>) -> Self {
        Self {
            driver_core: builder.build_core()
        }
    }
}


impl Driver for StickyDriver {
    fn get_driver_core(&self) -> &DriverCore {
        &self.driver_core
    }

    fn get_driver_core_mut(&mut self) -> &mut DriverCore {
        &mut self.driver_core
    }

    fn fall(&mut self) -> BoardTransition {
        let points = self.get_game_core().get_active_tetrimino().get_points();
        let (added, mut transitions) = self.driver_core.fall();
        if added {
            // calculate falling points
            let falls = calculate_sticky_falls(self.get_game_core().get_board(), points);
            if !falls.is_empty() {
                transitions.add_points_falling(falls)
            }
        }

        transitions
    }

    fn fastfall(&mut self) -> BoardTransition {
        let mut points = self.get_game_core().get_active_tetrimino().get_points();
        let (translation, mut transitions) = self.driver_core.fastfall();
        points = points.into_iter().map(|p| p - Point::unit_y(translation)).collect::<Vec<_>>();
        let falls = calculate_sticky_falls(self.get_game_core().get_board(), points);
        if !falls.is_empty() {
            transitions.add_points_falling(falls);
        }
        
        transitions
    }

    fn finish_transition(&mut self, transition: BoardTransition) -> BoardTransition { 
        let (cleared_rows, _, mut new_transitions) = self.driver_core.finish_transition(transition);

        if let Some(rows) = cleared_rows {
            let falls = calculate_sticky_falls_from_rows(self.driver_core.core.get_board(), rows);
            if !falls.is_empty() {
                new_transitions.add_points_falling(falls)
            }
        }

        new_transitions
    }
}