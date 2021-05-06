use crate::drivers::Driver;
use crate::game_core::GameCore;
use crate::game_core::utils::point::Point;
use crate::game_core::utils::orientations::Direction;


pub struct BaseDriver<'a> {
    core: GameCore<'a>,

    gravity_frames_per_cell_per_level: &'static [usize],
    frames_since_drop: usize,

    level: usize,
    score: usize,

    lock_delay: f32,
}

impl<'a> BaseDriver<'a> {
    pub fn new(core: GameCore<'a>, gravity_table: &'static [usize], lock_delay: f32) -> Self {
        Self {
            core,

            gravity_frames_per_cell_per_level: gravity_table,
            frames_since_drop: 0,

            level: 0,
            score: 0,

            lock_delay,
        }
    }
}

impl<'a> Driver<'a> for BaseDriver<'a> {
    fn get_game_core(&self) -> &GameCore<'a> {
        &self.core
    }

    fn get_game_core_mut(&mut self) -> &mut GameCore<'a> {
        &mut self.core
    }

    fn next_frame(&mut self) {
        self.frames_since_drop += 1;
        if self.frames_since_drop >= self.gravity_frames_per_cell_per_level[self.level] {
            self.core.fall();
            self.frames_since_drop = 0;
        }
    }
}
