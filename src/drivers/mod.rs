use rand::{thread_rng, Rng};

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

pub use utils::board_transition::*;


pub trait Driver {
    fn get_driver_core(&self) -> &DriverCore;
    fn get_driver_core_mut(&mut self) -> &mut DriverCore;

    /*
     * Basic getters.
     */
    fn get_game_core(&self) -> &GameCore {
        &self.get_driver_core().core
    }
    fn get_game_core_mut(&mut self) -> &mut GameCore {
        &mut self.get_driver_core_mut().core
    }
    fn get_score(&self) -> usize {
        self.get_driver_core().score
    }
    fn get_level(&self) -> usize {
        self.get_driver_core().level
    }

    /*
     * Engine for getting the next frame.
     */
    fn next_frame(&mut self) -> BoardTransition {
        if self.get_driver_core_mut().process_frame() {
            self.fall()
        } else {
            BoardTransition::new()
        }
    }

    /*
     * Controllable interactions.
     */
    fn translate_left(&mut self) {
        self.get_driver_core_mut().translate_left();
    }

    fn translate_right(&mut self) {
        self.get_driver_core_mut().translate_right();
    }

    fn rotate_clockwise(&mut self) {
        self.get_driver_core_mut().rotate_clockwise();
    }

    fn rotate_counterclockwise(&mut self) {
        self.get_driver_core_mut().rotate_counterclockwise();
    }
    fn hold(&mut self) {
        self.get_driver_core_mut().hold()
    }

    fn fall(&mut self) -> BoardTransition {
        self.get_driver_core_mut().fall().1
    }

    fn fastfall(&mut self) -> BoardTransition {
        self.get_driver_core_mut().fastfall().1
    }


    fn finish_transition(&mut self, _: BoardTransition) -> BoardTransition;
}

/**
 * Contains basic functionality that all drivers will share.
 */
pub struct DriverCore {
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

impl DriverCore {
    /**
     * Processes another frame, and returns a boolean indicating if a piece should fall 
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
                if !self.core.try_fall() {
                    self.lock_delayed = true;
                    self.frames_since_lock_delay = 0;
                    break;
                }
            }
        }

        false
    }

    fn translate_left(&mut self) -> bool {
        self.core.translate(Point(-1, 0))
    }

    fn translate_right(&mut self) -> bool {
        self.core.translate(Point(1, 0))
    }

    fn rotate_clockwise(&mut self) -> bool {
        self.core.rotate(Direction::Clockwise)
    }

    fn rotate_counterclockwise(&mut self) -> bool {
        self.core.rotate(Direction::CounterClockwise)
    }

    fn hold(&mut self) {
        if self.can_hold {
            self.core.hold();
            self.can_hold = false;
            self.lock_delayed = false;
        }
    }

    fn fall(&mut self) -> (bool, BoardTransition) {
        self.lock_delayed = false;
        let tetrimino_points = self.core.get_active_tetrimino().get_points();
        let (added, rows_deleted) = self.core.fall();

        let mut transition = BoardTransition::new()
            .with_rows_deleted(rows_deleted.unwrap_or(Vec::new()));

        if added {
            self.can_hold = true;
            transition.add_points_added(tetrimino_points);
        }

        (added, transition)
    }

    fn fastfall(&mut self) -> (i32, BoardTransition) {
        self.lock_delayed = false;
        self.can_hold = true;
 
        let tetrimino_points = self.core.get_active_tetrimino().get_points();
        let (translation, rows_deleted) = self.core.fastfall();

        let transition = BoardTransition::new()
            .with_points_added(tetrimino_points)
            .with_rows_deleted(rows_deleted.unwrap_or(Vec::new()));

        (translation, transition)
    }

    fn finish_transition(&mut self, mut transition: BoardTransition) -> (Option<Vec<i32>>, Option<Vec<Point>>, BoardTransition) {
        let mut chain_transition = BoardTransition::new();

        let mut deleted_rows = None;
        if let Some(mut rows) = transition.get_rows_deleted().cloned() {
            let board = self.core.get_board_mut();
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

            let board = self.core.get_board_mut();
            board.clear_points(&points);
            deleted_points = Some(points);
        }

        if let Some(points) = transition.take_points_falling() {
            let board = self.core.get_board_mut();
            let full_rows = board.translate_falling_points(&points);

            chain_transition.add_rows_deleted(full_rows);
        }

        (deleted_rows, deleted_points, chain_transition)
    }
}

pub struct BasicGenerator {
    tetrimino_chooser: utils::tetrimino_chooser::TetriminoChooser
}

impl BasicGenerator {
    pub fn new(tetrimino_types: &'static [TetriminoType]) -> Box<Self> {
        Box::new(Self {
            tetrimino_chooser: utils::tetrimino_chooser::TetriminoChooser::new(tetrimino_types)
        })
    }
}

impl TetriminoGenerator for BasicGenerator {
    fn next(&mut self) -> Tetrimino {
        let (index, tetrimino_type) = self.tetrimino_chooser.choose_tetrimino_type();
        let values = vec![index as u32; 4];
        tetrimino_type.instance(values)
    }

    fn get_tetrimino_types(&self) -> &'static [TetriminoType] {
        self.tetrimino_chooser.get_tetrimino_types()
    }

    fn set_seed(&mut self, seed: Vec<u8>) {
        self.tetrimino_chooser.set_seed(seed);
    }
}

pub trait BuildableDriver {
    type Data: Default;

    fn initialize(builder: DriverBuilder<Self>) -> DriverBuilder<Self> where Self: Sized {
        builder
    }

    fn build(builder: DriverBuilder<Self>) -> Self where Self: Sized;
}

pub struct DriverBuilder<T: BuildableDriver> {
    width: usize,
    height: usize,
    queue_length: usize,
    lock_delay: usize,
    get_gravity: fn(usize) -> f32,
    rng_seed: Vec<u8>,
    tetrimino_generator: Option<Box<dyn TetriminoGenerator>>,

    cont: T::Data
}

impl<T: BuildableDriver> DriverBuilder<T> {
    pub fn new() -> Self {
        T::initialize(Self {
            width: defaults::dimensions::CELL_WIDTH,
            height: defaults::dimensions::CELL_HEIGHT,
            queue_length: defaults::settings::QUEUE_LENGTH,
            lock_delay: 120,
            get_gravity: defaults::gravity::calculate_gravity,
            rng_seed: (0..32).map(|_| thread_rng().gen::<u8>()).collect(),
            tetrimino_generator: None,

            cont: Default::default()
        })
    }

    pub fn build(self) -> T {
        T::build(self)
    }

    pub fn build_boxed(self) -> Box<T> {
        Box::new(self.build())
    }

    pub fn build_core(&mut self) -> DriverCore {
        let board = Board::new(self.width, self.height);

        let mut tetrimino_generator = self.tetrimino_generator
            .take()
            .unwrap_or(BasicGenerator::new(defaults::tetriminos::TETRIMINOS));
        
        tetrimino_generator.set_seed(self.rng_seed.clone());

        // initialize the game engine
        let core = GameCore::new(
            board,
            self.queue_length,
            tetrimino_generator);

        DriverCore {
            core,

            get_gravity: self.get_gravity,
            frames_since_drop: 0.0,

            level: 0,
            score: 0,

            lock_delay: self.lock_delay,
            frames_since_lock_delay: 0,
            lock_delayed: false,

            can_hold: true,
        }
    }

    pub fn configured(self, configurer: fn(DriverBuilder<T>) -> Self) -> Self {
        configurer(self)        
    }

    pub fn with_tetrimino_generator(mut self, tetrimino_generator: Box<dyn TetriminoGenerator>) -> Self {
        self.tetrimino_generator = Some(tetrimino_generator);
        self
    }

    pub fn with_rng_seed(mut self, rng_seed: Vec<u8>) -> Self {
        self.rng_seed = rng_seed;
        self
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

    pub fn _with_get_gravity(mut self, get_gravity: fn(usize) -> f32) -> Self {
        self.get_gravity = get_gravity;
        self
    }
}