use crate::drivers::*;
use crate::game_core::defaults;
use super::utils::recursive_physics::calculate_sticky_falls_from_rows;
use super::utils::tetrimino_chooser::TetriminoChooser;


pub struct CascadeGenerator {
    tetrimino_chooser: TetriminoChooser,
    current_index: usize,
}

impl CascadeGenerator {
    pub fn new(tetrimino_types: &'static [TetriminoType]) -> Box<Self> {
        Box::new(Self {
            tetrimino_chooser: TetriminoChooser::new(tetrimino_types),
            current_index: 0,
        })
    }
}

impl TetriminoGenerator for CascadeGenerator {
    fn next(&mut self) -> Tetrimino {
        let num_tetriminos = self.get_tetrimino_types().len();
        let (index, tetrimino_type) = self.tetrimino_chooser.choose_tetrimino_type();
        let values = vec![(index + (self.current_index * num_tetriminos)) as u32; 4];
        self.current_index += 1;
        tetrimino_type.instance(values)
    }

    fn get_tetrimino_types(&self) -> &'static [TetriminoType] {
        self.tetrimino_chooser.get_tetrimino_types()
    }
}

pub struct CascadeDriver {
    driver_core: DriverCore
}

impl BuildableDriver for CascadeDriver {
    type Data = ();

    fn initialize(builder: DriverBuilder<Self>) -> DriverBuilder<Self> {
        builder
            .with_tetrimino_generator(
                CascadeGenerator::new(
                    defaults::tetriminos::TETRIMINOS))
    }

    fn build(mut builder: DriverBuilder<Self>) -> Self {
        Self {
            driver_core: builder.build_core()
        }
    }
}

impl Driver for CascadeDriver {
    fn get_driver_core(&self) -> &DriverCore {
        &self.driver_core
    }

    fn get_driver_core_mut(&mut self) -> &mut DriverCore {
        &mut self.driver_core
    }

    fn finish_transition(&mut self, transition: BoardTransition) -> BoardTransition { 
        let (rows_deleted, _, mut new_transitions) = self.driver_core.finish_transition(transition);

        if let Some(rows) = rows_deleted {
            let falls = calculate_sticky_falls_from_rows(self.driver_core.core.get_board(), rows);
            if !falls.is_empty() {
                new_transitions.add_points_falling(falls);
            }
        }

        return new_transitions
    }
}