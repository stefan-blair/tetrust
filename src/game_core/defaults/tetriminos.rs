use crate::game_core::tetriminos::*;
use crate::game_core::utils::point::{PartialPoint, Point};


pub const I_TETRIMINO: TetriminoType = TetriminoType::new(
    all_orientations!(
        PartialPoint(-1.5, 0.5),
        PartialPoint(-0.5, 0.5),
        PartialPoint(0.5, 0.5),
        PartialPoint(1.5, 0.5)
    ),
    I_WALL_KICKS,
    Point(-2, 1),
    Point(4, 1)
);

pub const T_TETRIMINO: TetriminoType = TetriminoType::new(
    all_orientations!(
        PartialPoint(-1.0, 0.0),
        PartialPoint(0.0, 0.0),
        PartialPoint(1.0, 0.0),
        PartialPoint(0.0, 1.0)
    ),
    OTHER_WALL_KICKS,
    Point(-1, 1),
    Point(3, 2),
);

pub const O_TETRIMINO: TetriminoType = TetriminoType::new(
    all_orientations!(
        PartialPoint(-0.5, 0.5),
        PartialPoint(0.5, 0.5),
        PartialPoint(-0.5, -0.5),
        PartialPoint(0.5, -0.5)
    ),
    OTHER_WALL_KICKS,
    Point(-2, 0),
    Point(2, 2)
);

pub const S_TETRIMINO: TetriminoType = TetriminoType::new(
    all_orientations!(
        PartialPoint(1.0, 1.0),
        PartialPoint(0.0, 1.0),
        PartialPoint(0.0, 0.0),
        PartialPoint(-1.0, 0.0)
    ),
    OTHER_WALL_KICKS,
    Point(-1, 1),
    Point(3, 2)
);

pub const Z_TETRIMINO: TetriminoType = TetriminoType::new(
    all_orientations!(
        PartialPoint(-1.0, 1.0),
        PartialPoint(0.0, 1.0),
        PartialPoint(0.0, 0.0),
        PartialPoint(1.0, 0.0)
    ),
    OTHER_WALL_KICKS,
    Point(-1, 1),
    Point(3, 2)
);

pub const L_TETRIMINO: TetriminoType = TetriminoType::new(
    all_orientations!(
        PartialPoint(-1.0, 1.0),
        PartialPoint(1.0, 0.0),
        PartialPoint(0.0, 0.0),
        PartialPoint(-1.0, 0.0)
    ),
    OTHER_WALL_KICKS,
    Point(-1, 1),
    Point(3, 2)
);

pub const J_TETRIMINO: TetriminoType = TetriminoType::new(
    all_orientations!(
        PartialPoint(1.0, 1.0),
        PartialPoint(1.0, 0.0),
        PartialPoint(0.0, 0.0),
        PartialPoint(-1.0, 0.0)
    ),
    OTHER_WALL_KICKS,
    Point(-1, 1),
    Point(3, 2)
);

pub const TETRIMINOS: &[TetriminoType] = &[
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
