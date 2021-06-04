use crate::game_core::utils::point::*;


#[derive(Default, Clone, Debug)]
pub struct BoardTransition {
    points_deleted: Vec<Point>,
    rows_deleted: Vec<i32>,
    points_falling: Vec<(Point, i32)>,
    points_added: Vec<Point>
}

impl BoardTransition {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_points_deleted(&mut self, mut points_deleted: Vec<Point>) {
        self.points_deleted.append(&mut points_deleted)
    }

    pub fn add_rows_deleted(&mut self, mut rows_deleted: Vec<i32>) {
        self.rows_deleted.append(&mut rows_deleted)
    }

    pub fn add_points_falling(&mut self, mut points_falling: Vec<(Point, i32)>) {
        self.points_falling.append(&mut points_falling)
    }

    pub fn add_points_added(&mut self, mut points_added: Vec<Point>) {
        self.points_added.append(&mut points_added)
    }

    pub fn add_from_transition(&mut self, mut transition: BoardTransition) {
        self.points_deleted.append(&mut transition.points_deleted);
        self.rows_deleted.append(&mut transition.rows_deleted);
        self.points_falling.append(&mut transition.points_falling);
        self.points_added.append(&mut transition.points_added);
    }

    /**
     * Used to construct the transition using the builder pattern
     */
    pub fn _with_points_deleted(mut self, points_deleted: Vec<Point>) -> Self {
        self.points_deleted = points_deleted;
        self
    }

    pub fn with_rows_deleted(mut self, rows_deleted: Vec<i32>) -> Self {
        self.rows_deleted = rows_deleted;
        self
    }

    pub fn _with_points_falling(mut self, points_falling: Vec<(Point, i32)>) -> Self {
        self.points_falling = points_falling;
        self
    }

    pub fn with_points_added(mut self, points_added: Vec<Point>) -> Self {
        self.points_added = points_added;
        self
    }

    /**
     * Sorts and deduplicates all vectors of transitions.
     */
    pub fn compress(&mut self) {
        self.points_deleted.sort_by_key(|p| (p.y(), p.x()));
        self.points_deleted.dedup();    

        self.rows_deleted.sort();
        self.rows_deleted.dedup();    

        self.points_falling.sort_by_key(|(p, d)| (p.y(), *d, p.x()));
        self.points_falling.dedup();
    }

    pub fn get_points_deleted(&self) -> Option<&Vec<Point>> {
        if self.points_deleted.is_empty() {
            None
        } else {
            Some(&self.points_deleted)
        }
    }

    pub fn get_rows_deleted(&self) -> Option<&Vec<i32>> {
        if self.rows_deleted.is_empty() {
            None
        } else {
            Some(&self.rows_deleted)
        }
    }

    pub fn get_points_falling(&self) -> Option<&Vec<(Point, i32)>> {
        if self.points_falling.is_empty() {
            None
        } else {
            Some(&self.points_falling)
        }
    }

    pub fn get_points_added(&self) -> Option<&Vec<Point>> {
        if self.points_added.is_empty() {
            None
        } else {
            Some(&self.points_added)
        }
    }

    pub fn take_points_deleted(&mut self) -> Option<Vec<Point>> {
        if self.points_deleted.is_empty() {
            None
        } else {
            Some(std::mem::replace(&mut self.points_deleted, Vec::new()))
        }
    }

    pub fn take_rows_deleted(&mut self) -> Option<Vec<i32>> {
        if self.rows_deleted.is_empty() {
            None
        } else {
            Some(std::mem::replace(&mut self.rows_deleted, Vec::new()))
        }
    }

    pub fn take_points_falling(&mut self) -> Option<Vec<(Point, i32)>> {
        if self.points_falling.is_empty() {
            None
        } else {
            Some(std::mem::replace(&mut self.points_falling, Vec::new()))
        }
    }

    pub fn is_inert(&self) -> bool {
        self.points_deleted.is_empty() && 
        self.points_falling.is_empty() && 
        self.points_added.is_empty() &&
        self.rows_deleted.is_empty()
    }
}
