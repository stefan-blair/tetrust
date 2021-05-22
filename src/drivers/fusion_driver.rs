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
    driver_core: DriverCore,
    sink: Point,
}

#[derive(Default)]
pub struct FusionDriverBuilderData {
    sink: Point,
    junk: Vec<Point>
}

impl BuildableDriver for FusionDriver {
    type Data = FusionDriverBuilderData;

    fn initialize(builder: DriverBuilder<Self>) -> DriverBuilder<Self> {
        builder
            .with_tetrimino_generator(BasicGenerator::new(TETRIMINOS))
    }

    fn build(mut builder: DriverBuilder<Self>) -> Self where Self: Sized {
        // use the junk to populate the board
        Self {
            sink: builder.cont.sink,
            driver_core: builder
                .build_core()
        }
    }
}

impl DriverBuilder<FusionDriver> {
    pub fn _with_sink(mut self, sink: Point) -> Self {
        self.cont.sink = sink;
        self
    }

    pub fn _with_junk(mut self, junk: Vec<Point>) -> Self {
        self.cont.junk = junk;
        self
    }
}

impl FusionDriver {
    /**
     * For each transition, if a row is supposed to fall that contains fusion points,
     * that transition is replaced with a combo of deleting + falling points transitions
     * containing all of the points
     */
    fn extract_fusion_points(&self, mut transitions: BoardTransition) -> BoardTransition {
        let board = self.driver_core.core.get_board();

        if let Some(rows) = transitions.take_rows_deleted() {
            let mut deleted_rows = Vec::new();
            deleted_rows.reserve(rows.len());
            let mut deleted_points = Vec::new();
            deleted_points.reserve(rows.len() * board.get_width());

            for row in rows {
                // get all of the non-fusion points. if it does not total to the width, then a fusion piece was found
                let mut non_fusion_points = Vec::new();
                non_fusion_points.reserve(board.get_width());
                for point in (0..board.get_width()).map(|x| Point(x as i32, row)) {
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

            transitions.add_points_deleted(deleted_points);
            transitions.add_rows_deleted(deleted_rows);
        }

        transitions
    }
}

impl Driver for FusionDriver {
    fn get_driver_core(&self) -> &DriverCore {
        &self.driver_core
    }

    fn get_driver_core_mut(&mut self) -> &mut DriverCore {
        &mut self.driver_core
    }

    fn fall(&mut self) -> BoardTransition {
        let (added, transitions) = self.driver_core.fall();
        if added {
            self.extract_fusion_points(transitions)
        } else {
            transitions
        }
    }

    fn fastfall(&mut self) -> BoardTransition {
        let transition = self.driver_core.fastfall().1;
        self.extract_fusion_points(transition)
    }

    fn finish_transition(&mut self, transition: BoardTransition) -> BoardTransition { 
        let (cleared_rows, cleared_points, mut new_transitions) = self.driver_core.finish_transition(transition);

        if let Some(rows) = cleared_rows {
    //     // let falls = calculate_sticky_falls_from_rows(self.get_game_core().get_board(), rows);
    //     // if !falls.is_empty() {
    //     //     vec![BoardTransition::PointsFalling(falls)]
    //     // } else {
    //         BoardTransition::new()
    //     // }
        }

        if let Some(points) = cleared_points {
            let mut rows = points.into_iter().map(|p| p.y()).collect::<Vec<_>>();
            rows.sort();
            rows.dedup();

            let falls = calculate_sticky_falls_from_rows(self.get_game_core().get_board(), rows);
            new_transitions.add_points_falling(falls);
        }

        return self.extract_fusion_points(new_transitions);
    }
}