use crate::game_core::utils::orientations::{Direction, Orientation};
use crate::game_core::utils::point::{Point};


macro_rules! all_orientations {
    ( $ ( $ p : expr ) , * ) => {
        [
            &[$(
                $p.to_point(),
            )*],
            &[$(
                $p.rotate_clockwise().to_point(),
            )*],
            &[$(
                $p.rotate_clockwise().rotate_clockwise().to_point(),
            )*],
            &[$(
                $p.rotate_clockwise().rotate_clockwise().rotate_clockwise().to_point(),
            )*]
        ];
    };
}

type CellValueType = u32;

pub trait TetriminoGenerator {
    fn next(&mut self) -> Tetrimino;
}

pub struct TetriminoType {
    // an array of shapes, one for each orientation
    shapes: [&'static [Point]; Orientation::COUNT],
    // a table of wall kicks to attempt for each orientation -> orientation transition
    wall_kicks: [[&'static [Point]; Direction::COUNT]; Orientation::COUNT],
    // the top left point of the tetrimino's bounding box
    bounding_box: Point,
    // the width and height, in cells, of the tetriminio
    dimensions: Point,
}

impl TetriminoType {
    pub const fn new(
        shapes: [&'static [Point]; Orientation::COUNT],
        wall_kicks: [[&'static [Point]; Direction::COUNT]; Orientation::COUNT],
        bounding_box: Point,
        dimensions: Point,
    ) -> Self {
        Self {
            shapes,
            wall_kicks,
            bounding_box,
            dimensions,
        }
    }

    pub fn instance(&'static self, values: Vec<CellValueType>) -> Tetrimino {
        Tetrimino::new(self, values)
    }

    pub fn get_wall_kicks(&self, orientation: Orientation, direction: Direction) -> &[Point] {
        self.wall_kicks[orientation as usize][direction as usize]
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

#[derive(Clone)]
pub struct Tetrimino {
    pub tetrimino_type: &'static TetriminoType,
    // an array of numbers corresponding to each point.  could be used for color or something
    pub values: Vec<CellValueType>,
}

impl Tetrimino {
    fn new(tetrimino_type: &'static TetriminoType, values: Vec<CellValueType>) -> Self {
        Self {
            tetrimino_type,
            values
        }
    }

    pub fn as_active_instance(self, position: Point) -> ActiveTetrimino {
        ActiveTetrimino::new(position, self)
    }
}

#[derive(Clone)]
pub struct ActiveTetrimino {
    pub position: Point,
    pub orientation: Orientation,
    pub tetrimino: Tetrimino,
}

impl ActiveTetrimino {
    fn new(position: Point, tetrimino: Tetrimino) -> Self {
        Self {
            position,
            orientation: Orientation::Origin,
            tetrimino,
        }
    }

    pub fn translated(mut self, translation: Point) -> Self {
        self.position = self.position + translation;
        self
    }

    pub fn translate(&mut self, direction: Point) {
        self.position = self.position + direction
    }

    pub fn rotate(&mut self, direction: Direction) {
        self.orientation = self.orientation.rotated(direction);
    }

    pub fn get_points(&self) -> Vec<Point> {
        self.get_translated_points(Point(0, 0))
    }

    pub fn get_translated_points(&self, translation: Point) -> Vec<Point> {
        let tetrimino_type = self.tetrimino.tetrimino_type;
        let translation = self.position - tetrimino_type.bounding_box + translation;

        tetrimino_type.shapes[self.orientation as usize]
            .iter()
            .cloned()
            .map(|p| p + translation)
            .collect::<Vec<_>>()
    }

    pub fn get_rotated_points(&self, direction: Direction) -> Vec<Point> {
        let tetrimino_type = self.tetrimino.tetrimino_type;
        let translation = self.position - tetrimino_type.bounding_box;

        tetrimino_type.shapes[self.orientation.rotated(direction) as usize]
            .iter()
            .cloned()
            .map(|p| p + translation)
            .collect::<Vec<_>>()
    }

    pub fn get_tetrimino(&self) -> &Tetrimino {
        &self.tetrimino
    }
}
