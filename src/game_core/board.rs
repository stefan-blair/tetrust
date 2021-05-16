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

    fn get_row_count_mut(&mut self, index: i32) -> &mut usize {
        &mut self.cells[index as usize].1
    }

    pub fn get_cell(&self, point: Point) -> Cell {
        self.get_row(point.y() as usize)[point.x() as usize]
    }

    pub fn get_cell_mut(&mut self, point: Point) -> &mut Cell {
        &mut self.get_row_mut(point.y() as usize)[point.x() as usize]
    }

    pub fn fill_point(&mut self, point: Point, value: u32) -> bool {
        while (point.y() as usize) >= self.cells.len() {
            if self.cells.len() + 1 >= self.height {
                return false;
            }

            self.cells.push((vec![None; self.width], 0));
        }

        if !self.get_cell(point).is_some() {
            *self.get_row_count_mut(point.y()) += 1;
            self.get_row_mut(point.y() as usize)[point.x() as usize] = Some(value);
        }

        return true;
    }

    pub fn unfill_point(&mut self, point: Point) {
        let row_count = self.get_row_count_mut(point.y());
        *row_count -= 1;
        if *row_count == 0 && point.y() == self.num_active_rows() as i32 - 1 {
            let mut i = point.y();
            while *self.get_row_count_mut(i) == 0 {
                self.cells.remove(i as usize);
                i -= 1;
            }
        } else {
            *self.get_cell_mut(point) = None;
        }
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

    pub fn do_points_fit(&self, points: Vec<Point>) -> bool {
        for point in points {
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
            if !self.fill_point(point, tetrimino.get_tetrimino().values[i]) {
                return None;
            }

            if *self.get_row_count_mut(point.y()) == self.width {
                rows.push(point.y())
            }
        }

        Some(rows)
    }

    pub fn clear_rows(&mut self, mut rows: Vec<i32>) {
        rows.sort();
        let mut removed_rows = 0;
        for row in rows.into_iter() {
            if *self.get_row_count_mut(row - removed_rows) == self.width {
                self.cells.remove((row - removed_rows) as usize);
                removed_rows += 1;
            }
        }
    }

    pub fn clear_points(&mut self, points: &Vec<Point>) {
        for &point in points.iter() {
            self.unfill_point(point);
        }
    }

    pub fn translate_falling_points(&mut self, point_drops: &Vec<(Point, i32)>) -> Vec<i32> {
        let mut rows = Vec::new();
        for (point, fall) in point_drops.into_iter().cloned() {
            let cell = self.get_cell_mut(point);
            if let Some(value) = *cell {
                self.unfill_point(point);
                self.fill_point(point - Point::unit_y(fall), value);
            }

            if *self.get_row_count_mut(point.y() - fall) == self.width {
                rows.push(point.y() - fall)
            }
        }

        return rows
        // return rows that are now full!
    }

    pub fn point_first_collision(&self, point: Point) -> Point {
        let mut down_translation = point.y();
        for y in (-1..std::cmp::min(point.y() + 1, self.cells.len() as i32 + 1)).rev() {
            let dist = Point::unit_y(point.y() - y);
            if self.is_point_filled(point - dist) {
                down_translation = std::cmp::min(down_translation, dist.y() - 1);
            }
        }

        Point::unit_y(-down_translation)
    }

    pub fn first_collision(&self, points: Vec<Point>) -> Point {
        points
            .into_iter()
            .map(|p| self.point_first_collision(p))
            .max_by_key(|p| p.y())
            .unwrap()
    }

    pub fn num_active_rows(&self) -> usize {
        self.cells.len()
    }

    pub fn is_on_board(&self, point: Point) -> bool {
        if point.x() as usize >= self.width || point.x() < 0 {
            false
        } else if point.y() as usize >= self.height || point.y() < 0 {
            false
        } else {
            true
        }
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
