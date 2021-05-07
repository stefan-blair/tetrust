use crate::game_core::tetriminos;
use crate::game_core::utils::point::{PartialPoint, Point};

type WallKickTable = [[&'static [Point]; 2]; 4];
type TetriminoData = (&'static [PartialPoint], &'static [u32], Point, WallKickTable);

pub const I_TETRIMINO: TetriminoData = (
    &[
        PartialPoint(-1.5, 0.5),
        PartialPoint(-0.5, 0.5),
        PartialPoint(0.5, 0.5),
        PartialPoint(1.5, 0.5),
    ],
    &[1, 2, 1, 1],
    Point(-2, 1),
    I_WALL_KICKS,
);

pub const T_TETRIMINO: TetriminoData = (
    &[
        PartialPoint(-1.0, 0.0),
        PartialPoint(0.0, 0.0),
        PartialPoint(1.0, 0.0),
        PartialPoint(0.0, 1.0),
    ],
    &[1, 2, 2, 2],
    Point(-1, 1),
    OTHER_WALL_KICKS,
);

pub const O_TETRIMINO: TetriminoData = (
    &[
        PartialPoint(-0.5, 0.5),
        PartialPoint(0.5, 0.5),
        PartialPoint(-0.5, -0.5),
        PartialPoint(0.5, -0.5),
    ],
    &[1, 1, 2, 1],
    Point(-2, 0),
    OTHER_WALL_KICKS,
);

pub const S_TETRIMINO: TetriminoData = (
    &[
        PartialPoint(1.0, 1.0),
        PartialPoint(0.0, 1.0),
        PartialPoint(0.0, 0.0),
        PartialPoint(-1.0, 0.0),
    ],
    &[2, 2, 1, 1],
    Point(-1, 1),
    OTHER_WALL_KICKS,
);

pub const Z_TETRIMINO: TetriminoData = (
    &[
        PartialPoint(-1.0, 1.0),
        PartialPoint(0.0, 1.0),
        PartialPoint(0.0, 0.0),
        PartialPoint(1.0, 0.0),
    ],
    &[1, 2, 1, 1],
    Point(-1, 1),
    OTHER_WALL_KICKS,
);

pub const L_TETRIMINO: TetriminoData = (
    &[
        PartialPoint(-1.0, 1.0),
        PartialPoint(1.0, 0.0),
        PartialPoint(0.0, 0.0),
        PartialPoint(-1.0, 0.0),
    ],
    &[2, 2, 1, 1],
    Point(-1, 1),
    OTHER_WALL_KICKS,
);

pub const J_TETRIMINO: TetriminoData = (
    &[
        PartialPoint(1.0, 1.0),
        PartialPoint(1.0, 0.0),
        PartialPoint(0.0, 0.0),
        PartialPoint(-1.0, 0.0),
    ],
    &[2, 2, 1, 1],
    Point(-1, 1),
    OTHER_WALL_KICKS,
);

pub const TETRIMINOS: &[TetriminoData] = &[
    I_TETRIMINO,
    T_TETRIMINO,
    O_TETRIMINO,
    S_TETRIMINO,
    Z_TETRIMINO,
    L_TETRIMINO,
    J_TETRIMINO,
];

pub const I_WALL_KICKS: [[&[Point]; 2]; 4] = [
    // Origin
    // Clockwise, Counterclockwise
    [
        &[Point(-2, 0), Point(1, 0), Point(-2, -1), Point(1, 2)],
        &[Point(-1, 0), Point(2, 0), Point(-1, 2), Point(2, -1)],
    ],
    // Right
    [
        &[Point(-1, 0), Point(2, 0), Point(-1, 2), Point(2, -1)],
        &[Point(2, 0), Point(-1, 0), Point(2, 1), Point(-1, -2)],
    ],
    // Around
    [
        &[Point(2, 0), Point(-1, 0), Point(2, 1), Point(-1, -2)],
        &[Point(1, 0), Point(-2, 0), Point(1, -2), Point(-2, 1)],
    ],
    // Left
    [
        &[Point(1, 0), Point(-2, 0), Point(1, -2), Point(-2, 1)],
        &[Point(-2, 0), Point(1, 0), Point(-2, -1), Point(1, 2)],
    ],
];

pub const OTHER_WALL_KICKS: [[&[Point]; 2]; 4] = [
    // Origin
    // Clockwise, Counterclockwise
    [
        &[Point(-1, 0), Point(-1, 1), Point(0, -2), Point(-1, -2)],
        &[Point(1, 0), Point(1, 1), Point(0, -2), Point(1, -2)],
    ],
    // Right
    [
        &[Point(1, 0), Point(1, -1), Point(0, 2), Point(1, 2)],
        &[Point(1, 0), Point(1, -1), Point(0, 2), Point(1, 2)],
    ],
    // Around
    [
        &[Point(1, 0), Point(1, 1), Point(0, -2), Point(1, -2)],
        &[Point(-1, 0), Point(-1, 1), Point(0, -2), Point(-1, -2)],
    ],
    // Left
    [
        &[Point(-1, 0), Point(-1, -1), Point(0, 2), Point(-1, 2)],
        &[Point(-1, 0), Point(-1, -1), Point(0, 2), Point(-1, 2)],
    ],
];

pub fn tetrimino_types() -> Vec<tetriminos::Tetrimino> {
    TETRIMINOS
        .iter()
        .map(|(shape, values, bounding_box, wall_kicks)| {
            tetriminos::Tetrimino::new(shape, values.to_vec(), *wall_kicks, *bounding_box)
        })
        .collect::<Vec<_>>()
}
