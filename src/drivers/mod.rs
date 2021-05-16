use crate::game_core::GameCore;
use crate::game_core::utils::point::Point;
use crate::game_core::utils::orientations::Direction;
use crate::game_core::tetriminos::*;
use crate::game_core::board::Board;
use crate::game_core::defaults;

pub mod utils;
pub mod classic_driver;
pub mod sticky_driver;
pub mod cascade_driver;
pub mod fusion_driver;


#[derive(Default, Clone, Debug)]
pub struct BoardTransition {
    points_deleted: Option<Vec<Point>>,
    rows_deleted: Option<Vec<i32>>,
    points_falling: Option<Vec<(Point, i32)>>
}

impl BoardTransition {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_to_transition(&mut self, mut other: BoardTransition) {
        if let Some(mut other_points_deleted) = other.points_deleted.take() {
            if let Some(points_deleted) = self.points_deleted.as_mut() {
                points_deleted.append(&mut other_points_deleted);
            } else {
                self.points_deleted = Some(other_points_deleted);
            }
        }

        if let Some(mut other_rows_deleted) = other.rows_deleted.take() {
            if let Some(rows_deleted) = self.rows_deleted.as_mut() {
                rows_deleted.append(&mut other_rows_deleted);
            } else {
                self.rows_deleted = Some(other_rows_deleted);
            }
        }

        if let Some(mut other_points_falling) = other.points_falling.take() {
            if let Some(points_falling) = self.points_falling.as_mut() {
                points_falling.append(&mut other_points_falling);
            } else {
                self.points_falling = Some(other_points_falling);
            }
        }
    }

    pub fn with_points_deleted(mut self, points_deleted: Vec<Point>) -> Self {
        if !points_deleted.is_empty() {
            self.points_deleted = Some(points_deleted);
        }
        self
    }

    pub fn with_rows_deleted(mut self, rows_deleted: Vec<i32>) -> Self {
        if !rows_deleted.is_empty() {
            self.rows_deleted = Some(rows_deleted);
        }
        self
    }

    pub fn with_points_falling(mut self, points_falling: Vec<(Point, i32)>) -> Self {
        if !points_falling.is_empty() {
            self.points_falling = Some(points_falling);
        }
        self
    }

    /**
     * Sorts and deduplicates all vectors of transitions.
     */
    pub fn compress(&mut self) {
        if let Some(points_deleted) = self.points_deleted.as_mut() {
            points_deleted.sort_by_key(|p| (p.y(), p.x()));
            points_deleted.dedup();    
        }

        if let Some(rows_deleted) = self.rows_deleted.as_mut() {
            rows_deleted.sort();
            rows_deleted.dedup();    
        }

        if let Some(points_falling) = self.points_falling.as_mut() {
            points_falling.sort_by_key(|(p, d)| (p.y(), *d, p.x()));
            points_falling.dedup();    
        }
    }

    pub fn get_points_deleted(&self) -> Option<&Vec<Point>> {
        self.points_deleted.as_ref()
    }

    pub fn get_rows_deleted(&self) -> Option<&Vec<i32>> {
        self.rows_deleted.as_ref()
    }

    pub fn get_points_falling(&self) -> Option<&Vec<(Point, i32)>> {
        self.points_falling.as_ref()
    }

    pub fn take_points_deleted(&mut self) -> Option<Vec<Point>> {
        self.points_deleted.take()
    }

    pub fn take_rows_deleted(&mut self) -> Option<Vec<i32>> {
        self.rows_deleted.take()
    }

    pub fn take_points_falling(&mut self) -> Option<Vec<(Point, i32)>> {
        self.points_falling.take()
    }

    pub fn is_inert(&self) -> bool {
        self.points_deleted.is_none() && self.points_falling.is_none() && self.rows_deleted.is_none()
    }
}

pub struct BasicGenerator {
    tetrimino_type_chooser: utils::tetrimino_chooser::TetriminoChooser
}

impl BasicGenerator {
    pub fn new(tetrimino_types: &'static [TetriminoType]) -> Box<Self> {
        Box::new(Self {
            tetrimino_type_chooser: utils::tetrimino_chooser::TetriminoChooser::new(tetrimino_types)
        })
    }
}

impl TetriminoGenerator for BasicGenerator {
    fn next(&mut self) -> Tetrimino {
        let (index, tetrimino_type) = self.tetrimino_type_chooser.choose_tetrimino_type();
        let values = vec![index as u32; 4];
        tetrimino_type.instance(values)
    }
}

pub trait Driver {
    fn get_generator(tetrimino_types: &'static [TetriminoType]) -> Box<dyn TetriminoGenerator> where Self: Sized{
        BasicGenerator::new(tetrimino_types)
    }

    fn get_game_core(&self) -> &GameCore;
    fn get_game_core_mut(&mut self) -> &mut GameCore;
    fn get_score(&self) -> usize;
    fn get_level(&self) -> usize;

    fn next_frame(&mut self) -> BoardTransition;

    fn translate_left(&mut self) -> bool {
        self.get_game_core_mut().translate(Point(-1, 0))
    }

    fn translate_right(&mut self) -> bool {
        self.get_game_core_mut().translate(Point(1, 0))
    }

    fn rotate_clockwise(&mut self) -> bool {
        self.get_game_core_mut().rotate(Direction::Clockwise)
    }

    fn rotate_counterclockwise(&mut self) -> bool {
        self.get_game_core_mut().rotate(Direction::CounterClockwise)
    }
    
    fn hold(&mut self);
    fn fall(&mut self) -> (bool, BoardTransition);
    fn fastfall(&mut self) -> (i32, BoardTransition);
    fn rows_cleared(&mut self, _: Vec<i32>) -> BoardTransition;
    fn points_cleared(&mut self, points: Vec<Point>) -> BoardTransition;
    fn points_fell(&mut self, points: Vec<(Point, i32)>, full_rows: Vec<i32>) -> BoardTransition;

    fn finish_transition(&mut self, mut transition: BoardTransition) -> BoardTransition{
        let mut chain_transition = BoardTransition::new();

        let mut deleted_rows = None;
        if let Some(mut rows) = transition.get_rows_deleted().cloned() {
            let board = self.get_game_core_mut().get_board_mut();
            board.clear_rows(rows.clone());
            rows.sort();
            for i in 0..rows.len() {
                rows[i] -= i as i32;
            }
            
            deleted_rows = Some(rows);
        }

        let mut deleted_points = None;
        if let Some(mut points) = transition.take_points_deleted() {
            if let Some(mut rows) = transition.take_rows_deleted() {
                rows.sort();
                for point in points.iter_mut() {
                    // get the total number of rows below the current point
                    let num_rows_below = rows
                        .iter()
                        .enumerate()
                        .find(|(_, &r)| r >= point.y())
                        .map_or(rows.len(), |(i, _)| i);

                    // adjust the points down, now that the rows were deleted
                    *point = Point(point.x(), point.y() - num_rows_below as i32);
                }
            }

            let board = self.get_game_core_mut().get_board_mut();
            board.clear_points(&points);
            deleted_points = Some(points);
        }

        if let Some(points) = transition.take_points_falling() {
            let board = self.get_game_core_mut().get_board_mut();
            let rows = board.translate_falling_points(&points);

            chain_transition.add_to_transition(self.points_fell(points, rows));
        }

        if let Some(rows) = deleted_rows {
            chain_transition.add_to_transition(self.rows_cleared(rows));
        }

        if let Some(points) = deleted_points {
            chain_transition.add_to_transition(self.points_cleared(points));
        }

        chain_transition
    }
}

/**
 * Default implementation of the Driver trait. Classic tetris can directly use
 * this driver. Can be extended / wrapped for more interesting versions of
 * tetris, or ignored completely.
 */
pub struct DefaultDriver {
    core: GameCore,

    frames_since_drop: f32,
    get_gravity: fn(usize) -> f32,

    level: usize,
    score: usize,

    lock_delay: usize,
    frames_since_lock_delay: usize,
    lock_delayed: bool,

    can_hold: bool
}

impl DefaultDriver {
    pub fn new(core: GameCore, get_gravity: fn(usize) -> f32, lock_delay: usize) -> Self {
        Self {
            core,

            get_gravity,
            frames_since_drop: 0.0,

            level: 0,
            score: 0,

            lock_delay,
            frames_since_lock_delay: 0,
            lock_delayed: false,

            can_hold: true,
        }
    }

    /**
     * Processes another frame, and returns a boolean indicating if the 
     */
    pub fn process_frame(&mut self) -> bool {
        if self.lock_delayed {
            self.frames_since_lock_delay += 1;
            if self.frames_since_lock_delay > self.lock_delay {
                self.lock_delayed = false;
                return true
            }            
        } else {
            self.frames_since_drop += 1.0;
            let gravity = (self.get_gravity)(self.level);
            while self.frames_since_drop > gravity {
                self.frames_since_drop = 0.0;
                if !self.get_game_core_mut().try_fall() {
                    self.lock_delayed = true;
                    self.frames_since_lock_delay = 0;
                    break;
                }
            }
        }

        false
    }
}

impl Driver for DefaultDriver {
    fn get_game_core(&self) -> &GameCore {
        &self.core
    }
    fn get_game_core_mut(&mut self) -> &mut GameCore {
        &mut self.core
    }

    fn get_score(&self) -> usize {
        self.score
    }

    fn get_level(&self) -> usize {
        self.level
    }

    fn next_frame(&mut self) -> BoardTransition {
        if self.process_frame() {
            self.fall().1
        } else {
            BoardTransition::new()
        }
    }

    fn hold(&mut self) {
        if self.can_hold {
            self.core.hold();
            self.can_hold = false;
        }
    }

    fn fall(&mut self) -> (bool, BoardTransition) {
        self.lock_delayed = false;
        let game_core = self.get_game_core_mut();
        let (added, rows) = game_core.fall();
        if let Some(rows) = rows {
            if !rows.is_empty() {
                return (true, BoardTransition::new().with_rows_deleted(rows));
            }
        }

        if added {
            self.can_hold = true;
        }

        (added, BoardTransition::new())
    }

    fn fastfall(&mut self) -> (i32, BoardTransition) {
        self.lock_delayed = false;
        self.can_hold = true;
 
        let game_core = self.get_game_core_mut();
        let (translation, rows) = game_core.fastfall();
        if let Some(rows) = rows {
            if !rows.is_empty() {
                return (translation, BoardTransition::new().with_rows_deleted(rows))
            }
        }
        (translation, BoardTransition::new())
    }

    fn rows_cleared(&mut self, _rows: Vec<i32>) -> BoardTransition {
        BoardTransition::new()
    }

    fn points_cleared(&mut self, _points: Vec<Point>) -> BoardTransition {
        BoardTransition::new()
    }

    fn points_fell(&mut self, _points: Vec<(Point, i32)>, full_rows: Vec<i32>) -> BoardTransition {
        BoardTransition::new()
            .with_rows_deleted(full_rows)
    }
}

pub struct DefaultDriverBuilder {
    width: usize,
    height: usize,
    queue_length: usize,
    tetriminos: &'static[TetriminoType],
    lock_delay: usize,
    get_gravity: fn(usize) -> f32
}

impl DefaultDriverBuilder {
    pub fn new() -> Self {
        Self {
            width: defaults::dimensions::CELL_WIDTH,
            height: defaults::dimensions::CELL_HEIGHT,
            queue_length: defaults::settings::QUEUE_LENGTH,
            tetriminos: defaults::tetriminos::TETRIMINOS,
            lock_delay: 120,
            get_gravity: defaults::gravity::calculate_gravity
        }
    }

    pub fn build(&self) -> DefaultDriver {
        let board = Board::new(self.width, self.height);
        // initialize the game engine
        let core = GameCore::new(
            self.tetriminos,
            board,
            self.queue_length,
            BasicGenerator::new(self.tetriminos));

        DefaultDriver::new(core, self.get_gravity, self.lock_delay)
    }

    pub fn _with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn _with_height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }

    pub fn _with_queue_length(mut self, queue_length: usize) -> Self {
        self.queue_length = queue_length;
        self
    }

    pub fn _with_lock_delay(mut self, lock_delay: usize) -> Self {
        self.lock_delay = lock_delay;
        self
    }

    pub fn with_tetriminos(mut self, tetriminos: &'static [TetriminoType]) -> Self {
        self.tetriminos = tetriminos;
        self
    }

    pub fn _with_get_gravity(mut self, get_gravity: fn(usize) -> f32) -> Self {
        self.get_gravity = get_gravity;
        self
    }
}