use crate::game_core::GameCore;


pub struct Driver<'a> {
    pub core: GameCore<'a>,

    gravity_frames_per_cell_per_level: &'static [usize],
    frames_since_drop: usize,

    level: usize,
    score: usize,

    lock_delay: f32,
}

impl<'a> Driver<'a> {
    pub fn new(
        core: GameCore<'a>, 
        gravity_table: &'static[usize],
        lock_delay: f32
    ) -> Self {
        Self {
            core,

            gravity_frames_per_cell_per_level: gravity_table,
            frames_since_drop: 0,

            level: 0,
            score: 0,

            lock_delay
        }
    }

    pub fn next_frame(&mut self) {
        self.frames_since_drop += 1;
        if self.frames_since_drop >= self.gravity_frames_per_cell_per_level[self.level] {
            self.core.fall();
            self.frames_since_drop = 0;
        }
    }
}