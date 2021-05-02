use crate::game::point::Point;
use crate::game::tetriminos;

pub struct Board {
    cells: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: Vec::new(),
            width,
            height,
        }
    }

    pub fn fill_point(&mut self, point: Point) -> bool {
        while (point.y() as usize) >= self.cells.len() {
            if self.cells.len() + 1 >= self.height {
                return false;
            }

            self.cells.push(vec![false; self.width]);
        }

        self.cells[point.y() as usize][point.x() as usize] = true;
        return true;
    }

    pub fn is_point_filled(&self, point: Point) -> bool {
        if (point.y() as usize) < self.cells.len() && point.y() >= 0 {
            if point.x() >= 0 && (point.x() as usize) < self.width {
                self.cells[point.y() as usize][point.x() as usize]
            } else {
                true
            }
        } else if point.y() < 0 {
            true
        } else {
            false
        }
    }

    pub fn does_tetrimino_fit(&self, tetrimino: tetriminos::ActiveTetrimino) -> bool {
        for point in tetrimino.get_points() {
            if self.is_point_filled(point) {
                return false;
            }
        }

        return true;
    }

    pub fn add_tetrimino(&mut self, tetrimino: tetriminos::ActiveTetrimino) -> bool {
        for point in tetrimino.get_points() {
            if !self.fill_point(point) {
                return false;
            }
        }

        return true;
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_spawn_point(&self) -> Point {
        Point::new(self.width as i32 / 2, self.height as i32)
    }
}
