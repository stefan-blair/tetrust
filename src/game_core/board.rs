use crate::game_core::tetriminos;
use crate::game_core::utils::point::Point;


type Cell = Option<u32>;

pub struct Board {
    cells: Vec<(Vec<Cell>, usize)>,
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

    fn get_row_mut(&mut self, index: usize) -> &mut Vec<Cell> {
        &mut self.cells[index].0
    }

    fn get_row(&self, index: usize) -> &Vec<Cell> {
        &self.cells[index].0
    }

    fn get_row_count_mut(&mut self, index: usize) -> &mut usize {
        &mut self.cells[index].1
    }

    pub fn get_row_count(&self, index: usize) -> usize {
        if index < self.cells.len() {
            self.cells[index].1
        } else {
            0
        }
    }

    pub fn get_cell(&self, point: Point) -> Cell {
        self.get_row(point.y() as usize)[point.x() as usize]
    }

    pub fn fill_point(&mut self, point: Point, value: u32) -> bool {
        while (point.y() as usize) >= self.cells.len() {
            if self.cells.len() + 1 >= self.height {
                return false;
            }

            self.cells.push((vec![None; self.width], 0));
        }

        if !self.get_cell(point).is_some() {
            *self.get_row_count_mut(point.y() as usize) += 1;
            self.get_row_mut(point.y() as usize)[point.x() as usize] = Some(value);
        }

        return true;
    }

    pub fn is_point_filled(&self, point: Point) -> bool {
        if point.x() < 0 || (point.x() as usize) >= self.width {
            true
        } else if (point.y() as usize) < self.cells.len() && point.y() >= 0 {
            self.get_cell(point).is_some()
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

    /**
     * Returns a vector of rows that are full and should be removed.
     * If adding the tetrimino would overflow the board's stack, None is returned.
     */
    pub fn add_tetrimino(&mut self, tetrimino: tetriminos::ActiveTetrimino) -> Option<Vec<i32>> {
        let mut rows = Vec::new();

        for (i, point) in tetrimino.get_points().into_iter().enumerate() {
            if !self.fill_point(point, tetrimino.get_tetrimino().get_values()[i]) {
                return None;
            }

            if *self.get_row_count_mut(point.y() as usize) == self.width {
                rows.push(point.y())
            }
        }

        Some(rows)
    }

    pub fn clear_rows(&mut self, mut rows: Vec<i32>) {
        rows.sort();
        let mut removed_rows = 0;
        for row in rows.into_iter().map(|i| i as usize) {
            if *self.get_row_count_mut(row - removed_rows) == self.width {
                self.cells.remove(row - removed_rows);
                removed_rows += 1;
            }
        }
    }

    pub fn first_collision(&self, tetrimino: tetriminos::ActiveTetrimino) -> Point {
        let mut down_translation = tetrimino.position.y();
        for point in tetrimino.get_points() {
            for y in (-1..std::cmp::min(point.y() + 1, self.cells.len() as i32 + 1)).rev() {
                let dist = Point::unit_y(point.y() - y);
                if self.is_point_filled(point - dist) {
                    down_translation = std::cmp::min(down_translation, dist.y() - 1);
                }
            }
        }

        Point::unit_y(-down_translation)
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_spawn_point(&self) -> Point {
        Point::new(self.width as i32 / 2, self.height as i32 - 1)
    }
}
