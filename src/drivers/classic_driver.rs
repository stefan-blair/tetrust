use crate::drivers::*;


pub struct ClassicDriver {
    last_clear_was_tetris: bool,
    driver_core: DriverCore,
}

impl ClassicDriver {
    fn update_score(&mut self, increment: usize) {
        self.driver_core.score += increment;
        let level = self.driver_core.score / 5;
        if self.driver_core.level < level && level < 15 {
            self.driver_core.level = level;
        }
    }
}

impl BuildableDriver for ClassicDriver {
    type Data = ();

    fn build(mut builder: DriverBuilder<Self>) -> Self where Self: Sized {
        ClassicDriver {
            last_clear_was_tetris: false,
            driver_core: builder.build_core()
        }
    }
}

impl Driver for ClassicDriver {
    fn get_driver_core(&self) -> &DriverCore {
        &self.driver_core
    }

    fn get_driver_core_mut(&mut self) -> &mut DriverCore {
        &mut self.driver_core
    }

    fn finish_transition(&mut self, transition: BoardTransition) -> BoardTransition { 
        let (cleared_rows, _, new_transition) = self.driver_core.finish_transition(transition); 

        if let Some(rows) = cleared_rows {
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
        }

        new_transition
    }
}