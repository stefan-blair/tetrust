use crate::game_core::tetriminos::*;
use crate::game_core::utils::point::*;
use crate::game_core::defaults::tetriminos::*;
use super::utils::recursive_physics::calculate_sticky_falls_from_rows;
use super::*;


pub const FUSION_TETRIMINO: TetriminoType = TetriminoType::new(
    all_orientations!(
        PartialPoint(0.0, 0.0)
    ),
    OTHER_WALL_KICKS,
    Point(0, 0),
    Point(1, 1)
);

pub const TETRIMINOS: &[TetriminoType] = &[
    I_TETRIMINO,
    T_TETRIMINO,
    O_TETRIMINO,
    S_TETRIMINO,
    Z_TETRIMINO,
    L_TETRIMINO,
    J_TETRIMINO,
    FUSION_TETRIMINO,
];

pub const FUSION_TETRIMINO_INDEX: usize = TETRIMINOS.len() - 1;

pub struct FusionDriver {
    default_driver: DefaultDriver,
    sink: Point,
}

impl Default for FusionDriver {
    fn default() -> Self {
        Self {
            sink: Point(0, 0),
            default_driver: DefaultDriverBuilder::new()
                .with_tetriminos(TETRIMINOS)
                .build()
        }
    }
}

impl FusionDriver {
    /**
     * For each transition, if a row is supposed to fall that contains fusion points,
     * that transition is replaced with a combo of deleting + falling points transitions
     * containing all of the points
     */
    fn extract_fusion_points(&self, mut transitions: BoardTransition) -> BoardTransition {
        let board = self.default_driver.core.get_board();

        if let Some(rows) = transitions.take_rows_deleted() {
            let mut deleted_rows = Vec::new();
            deleted_rows.reserve(rows.len());
            let mut deleted_points = Vec::new();
            deleted_points.reserve(rows.len() * board.get_width());

            for row in rows {
                // get all of the non-fusion points. if it does not total to the width, then a fusion piece was found
                let mut non_fusion_points = Vec::new();
                non_fusion_points.reserve(board.get_width());
                for point in (0..board.get_width()).map(|x| Point(x as i32, row as i32)) {
                    if board.get_cell(point).unwrap() != FUSION_TETRIMINO_INDEX as u32 {
                        non_fusion_points.push(point);
                    }
                }

                if non_fusion_points.len() == board.get_width() {
                    deleted_rows.push(row);
                } else {
                    deleted_points.append(&mut non_fusion_points);
                }
            }

            transitions.add_to_transition(BoardTransition::new()
                .with_points_deleted(deleted_points)
                .with_rows_deleted(deleted_rows));
        }

        transitions
    }
}

impl Driver for FusionDriver {
    fn get_generator(tetrimino_types: &'static [TetriminoType]) -> Box<dyn TetriminoGenerator> {
        BasicGenerator::new(tetrimino_types)
    }

    fn get_game_core(&self) -> &GameCore {
        self.default_driver.get_game_core()
    }

    fn get_game_core_mut(&mut self) -> &mut GameCore {
        self.default_driver.get_game_core_mut()
    }

    fn get_score(&self) -> usize {
        self.default_driver.get_score()
    }

    fn get_level(&self) -> usize {
        self.default_driver.get_level()
    }

    fn hold(&mut self) {
        self.default_driver.hold()
    }

    fn next_frame(&mut self) -> BoardTransition {
        if self.default_driver.process_frame() {
            self.fall().1
        } else {
            BoardTransition::new()
        }
    }

    fn fall(&mut self) -> (bool, BoardTransition) {
        let (added, transitions) = self.default_driver.fall();
        (added, self.extract_fusion_points(transitions))
    }

    fn fastfall(&mut self) -> (i32, BoardTransition) {
        let (translation, transitions) = self.default_driver.fastfall();
        (translation, self.extract_fusion_points(transitions))
    }

    fn rows_cleared(&mut self, rows: Vec<i32>) -> BoardTransition {
        // let falls = calculate_sticky_falls_from_rows(self.get_game_core().get_board(), rows);
        // if !falls.is_empty() {
        //     vec![BoardTransition::PointsFalling(falls)]
        // } else {
            BoardTransition::new()
        // }
    }

    fn points_cleared(&mut self, points: Vec<Point>) -> BoardTransition {
        let mut rows = points.into_iter().map(|p| p.y()).collect::<Vec<_>>();
        rows.sort();
        rows.dedup();

        let falls = calculate_sticky_falls_from_rows(self.get_game_core().get_board(), rows);
        BoardTransition::new().with_points_falling(falls)
    }

    fn points_fell(&mut self, _points: Vec<(Point, i32)>, full_rows: Vec<i32>) -> BoardTransition {
        self.extract_fusion_points(BoardTransition::new().with_rows_deleted(full_rows))
    }
}