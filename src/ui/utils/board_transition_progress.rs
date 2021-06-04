use crate::drivers::utils::board_transition::BoardTransition;


const POINTS_DELETED_DURATION: usize = 10;
const ROWS_DELETED_DURATION: usize = 10;
const POINTS_FALLING_DURATION: usize = 10;
const POINTS_ADDED_DURATION: usize = 0;

#[derive(Clone, Copy, Debug)]
pub struct BoardTransitionsProgress {
    points_deleted_total: usize,
    rows_deleted_total: usize,
    points_falling_total: usize,
    points_added_total: usize,
    longest_total: usize,
    elapsed: usize
}

impl BoardTransitionsProgress {
    pub fn new() -> Self {
        Self {
            points_deleted_total: POINTS_DELETED_DURATION,
            rows_deleted_total: ROWS_DELETED_DURATION,
            points_falling_total: POINTS_FALLING_DURATION,
            points_added_total: POINTS_ADDED_DURATION,
            longest_total: 10,
            elapsed: 0
        }
    }

    fn with_recalculated_longest(mut self) -> Self {
        self.longest_total = vec![
            self.points_deleted_total, 
            self.rows_deleted_total, 
            self.points_falling_total, 
            self.points_added_total
        ].into_iter().max().unwrap();

        self
    }

    pub fn with_board_transition(mut self, board_transition: &BoardTransition) -> Self {
        if let None = board_transition.get_points_added() {
            self.points_added_total = 0
        }

        if let None = board_transition.get_points_deleted() {
            self.points_deleted_total = 0
        }

        if let None = board_transition.get_points_falling() {
            self.points_falling_total = 0;
        }

        if let None = board_transition.get_rows_deleted() {
            self.rows_deleted_total = 0;
        }

        self.with_recalculated_longest()
    }

    pub fn with_points_deleted_total(mut self, points_deleted_total: usize) -> Self {
        self.points_deleted_total = points_deleted_total;
        self.with_recalculated_longest()
    }

    pub fn with_rows_deleted_total(mut self, rows_deleted_total: usize) -> Self {
        self.rows_deleted_total = rows_deleted_total;
        self.with_recalculated_longest()
    }

    pub fn with_points_falling_total(mut self, points_falling_total: usize) -> Self {
        self.points_falling_total = points_falling_total;
        self.with_recalculated_longest()
    }

    pub fn with_points_added_total(mut self, points_added_total: usize) -> Self {
        self.points_added_total = points_added_total;
        self.with_recalculated_longest()
    }

    fn transition_progress(&self, total: usize) -> f32 {
        (self.elapsed as f32) / (total as f32)
    }

    pub fn points_deleted_progress(&self) -> f32 {
        self.transition_progress(self.points_deleted_total)
    }

    pub fn rows_deleted_progress(&self) -> f32 {
        self.transition_progress(self.rows_deleted_total)
    }

    pub fn points_falling_progress(&self) -> f32 {
        self.transition_progress(self.points_falling_total)
    }

    pub fn points_added_progress(&self) -> f32 {
        self.transition_progress(self.points_added_total)
    }

    pub fn next_frame(&mut self) {
        self.elapsed += 1
    }

    pub fn is_complete(&self) -> bool {
        self.elapsed > self.longest_total
    }
}