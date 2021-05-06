use crate::game_core::utils::orientations::{Direction, Orientation};
use crate::game_core::utils::point::{PartialPoint, Point};

pub struct Tetrimino {
    // an array of shapes, one for each orientation
    shapes: Vec<Vec<Point>>,
    // a table of wall kicks to attempt for each orientation -> orientation transition
    wall_kicks: [[&'static [Point]; Direction::COUNT]; Orientation::COUNT],
    // the top left point of the tetrimino's bounding box
    bounding_box: Point,
    // the width and height, in cells, of the tetriminio
    dimensions: Point,
}

impl Tetrimino {
    pub fn new(
        shape: &[PartialPoint],
        wall_kicks: [[&'static [Point]; Direction::COUNT]; Orientation::COUNT],
        bounding_box: Point,
    ) -> Self {
        // initialize the different shape rotations
        let mut shapes = Vec::new();
        let mut current_rotation = shape.to_vec();
        for _ in 0..Orientation::COUNT {
            // push the last shape shifted to regular points
            shapes.push(
                current_rotation
                    .iter()
                    .map(|PartialPoint(x, y)| Point(x.floor() as i32, y.floor() as i32))
                    .collect::<Vec<_>>(),
            );
            // update the current rotation by rotating each individual point
            let rotated = current_rotation
                .iter()
                .cloned()
                .map(|PartialPoint(x, y)| PartialPoint(y, -x))
                .collect::<Vec<_>>();
            current_rotation = rotated;
        }

        // calculate the dimensions of the tetrimino
        let (mut left, mut right, mut up, mut down) = (0, 0, 0, 0);
        for point in shapes[0].iter() {
            if point.x() < left {
                left = point.x();
            }
            if point.x() > right {
                right = point.x();
            }
            if point.y() < down {
                down = point.y();
            }
            if point.y() > up {
                up = point.y();
            }
        }

        let dimensions = Point(right - left + 1, up - down + 1);

        Self {
            shapes,
            wall_kicks,
            bounding_box,
            dimensions,
        }
    }

    pub fn active_instance(&self) -> ActiveTetrimino {
        ActiveTetrimino::new(Point::default(), self)
    }

    pub fn get_wall_kicks(&self, orientation: Orientation, direction: Direction) -> &[Point] {
        self.wall_kicks[orientation as usize][direction as usize]
    }

    pub fn get_bounding_box(&self) -> Point {
        self.bounding_box
    }

    pub fn get_points(&self) -> Vec<Point> {
        self.shapes[0]
            .iter()
            .cloned()
            .map(|p| p - self.bounding_box)
            .collect::<Vec<_>>()
    }

    pub fn get_dimensions(&self) -> Point {
        self.dimensions
    }
}

#[derive(Clone, Copy)]
pub struct ActiveTetrimino<'a> {
    pub position: Point,
    pub orientation: Orientation,
    tetrimino: &'a Tetrimino,
}

impl<'a> ActiveTetrimino<'a> {
    pub fn new(position: Point, tetrimino: &'a Tetrimino) -> Self {
        Self {
            position,
            orientation: Orientation::Origin,
            tetrimino,
        }
    }

    pub fn with_position(mut self, position: Point) -> Self {
        self.position = position;
        self
    }

    pub fn translated(mut self, translation: Point) -> Self {
        self.position = self.position + translation;
        self
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn rotated(mut self, direction: Direction) -> Self {
        self.orientation = self.orientation.rotated(direction);
        self
    }

    pub fn rotate_clockwise(&mut self) {
        self.orientation = self.orientation.rotated_clockwise();
    }

    pub fn rotate_counter_clockwise(&mut self) {
        self.orientation = self.orientation.rotated_counter_clockwise();
    }

    pub fn get_points(&self) -> Vec<Point> {
        let translation = self.position - self.tetrimino.bounding_box;

        self.tetrimino.shapes[self.orientation as usize]
            .iter()
            .cloned()
            .map(|p| p + translation)
            .collect::<Vec<_>>()
    }

    pub fn get_tetrimino(&self) -> &'a Tetrimino {
        self.tetrimino
    }

    // pub fn get_bounding_box(&self) -> Point {
    //     self.tetrimino.bounding_box + self.
    // }
}
