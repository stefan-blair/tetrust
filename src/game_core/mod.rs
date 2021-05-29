#[macro_use]
pub mod tetriminos;
pub mod board;
pub mod defaults;
pub mod utils;

use tetriminos::*;
use utils::point::Point;
use utils::orientations::*;


pub struct GameCore {
    active_tetrimino: ActiveTetrimino,
    ghost_tetrimino: Vec<Point>,
    held_tetrimino: Option<Tetrimino>,

    tetrimino_queue: Vec<Tetrimino>,
    next_tetrimino_index: usize,

    board: board::Board,

    tetrimino_generator: Box<dyn TetriminoGenerator>
}

impl GameCore {
    pub fn new(
        board: board::Board,
        queue_length: usize,
        mut tetrimino_generator: Box<dyn TetriminoGenerator>
    ) -> Self {
        let active_tetrimino = tetrimino_generator
            .next()
            .as_active_instance(board.get_spawn_point());

        let tetrimino_queue = (0..queue_length)
            .map(|_| tetrimino_generator.next())
            .collect::<Vec<_>>();

        Self {
            ghost_tetrimino: active_tetrimino.get_points(),
            active_tetrimino,
            held_tetrimino: None,
            tetrimino_queue,
            next_tetrimino_index: 0,
            board,
            tetrimino_generator,
        }
    }

    pub fn get_board(&self) -> &board::Board {
        &self.board
    }

    pub fn get_board_mut(&mut self) -> &mut board::Board {
        &mut self.board
    }

    pub fn get_active_tetrimino(&self) -> &ActiveTetrimino {
        &self.active_tetrimino
    }

    pub fn get_ghost_tetriminio(&self) -> &Vec<Point> {
        &self.ghost_tetrimino
    }

    pub fn get_next_tetrimino(&self, index: usize) -> &Tetrimino {
        &self.tetrimino_queue[(self.next_tetrimino_index + index) % self.tetrimino_queue.len()]
    }

    pub fn get_tetrimino_queue_length(&self) -> usize {
        self.tetrimino_queue.len()
    }

    pub fn get_tetrimino_types(&self) -> &[TetriminoType] {
        self.tetrimino_generator.get_tetrimino_types()
    }

    // returns the old active tetrimino
    pub fn set_active_tetrimino(&mut self, active_tetrimino: ActiveTetrimino) -> ActiveTetrimino {
        let old_active_tetrimino = std::mem::replace(&mut self.active_tetrimino, active_tetrimino);
        self.active_tetrimino_updated();

        old_active_tetrimino
    }

    // returns the old active tetrimino
    pub fn next_tetrimino(&mut self) -> ActiveTetrimino {
        let new_tetrimino = self.tetrimino_generator.next();
        let next_tetrimino = std::mem::replace(&mut self.tetrimino_queue[self.next_tetrimino_index], new_tetrimino);
        self.next_tetrimino_index = (self.next_tetrimino_index + 1) % self.tetrimino_queue.len();
        let old_active_tetrimino = self.set_active_tetrimino(next_tetrimino.as_active_instance(self.board.get_spawn_point()));

        old_active_tetrimino
    }

    pub fn active_tetrimino_updated(&mut self) {
        let points = self.active_tetrimino.get_points();
        let ghost_translation = self.board.first_collision(points);
        self.ghost_tetrimino = self.active_tetrimino.get_translated_points(ghost_translation);
    }

    pub fn hold(&mut self) {
        match &self.held_tetrimino {
            Some(_) => {
                let held_tetrimino = self.held_tetrimino
                    .take()
                    .unwrap()
                    .as_active_instance(self.board.get_spawn_point());
                let old_tetrimino = self.set_active_tetrimino(held_tetrimino).tetrimino;

                self.held_tetrimino = Some(old_tetrimino)
            },
            None => {
                self.held_tetrimino = Some(self.next_tetrimino().tetrimino)                
            }
        }
    }

    pub fn get_held(&self) -> Option<&Tetrimino> {
        self.held_tetrimino.as_ref()
    }

    pub fn translate(&mut self, direction: Point) -> bool {
        let translated_points = self.active_tetrimino.get_translated_points(direction);
        if self.board.do_points_fit(translated_points) {
            self.active_tetrimino.translate(direction);
            self.active_tetrimino_updated();

            true
        } else {
            false
        }
    }

    pub fn add_tetrimino(&mut self) -> Option<Vec<i32>> {
        let rows = self.board.add_tetrimino(self.active_tetrimino.clone());
        self.next_tetrimino();

        rows
    }

    // returns true if the active tetrimino successfully fell, false otherwise
    pub fn try_fall(&mut self) -> bool {
        self.translate(Point(0, -1))
    }

    pub fn fall(&mut self) -> (bool, Option<Vec<i32>>) {
        // if the piece can fall no further, then place it and get the next piece
        if !self.try_fall() {
            (true, self.add_tetrimino())
        } else {
            (false, None)
        }
    }

    pub fn fastfall(&mut self) -> (i32, Option<Vec<i32>>) {
        let translation = self.board.first_collision(self.active_tetrimino.get_points());
        self.active_tetrimino = self.active_tetrimino.clone().translated(translation);
        (-translation.y(), self.add_tetrimino())
    }

    pub fn rotate(&mut self, direction: Direction) -> bool {
        let oriented_points = self.active_tetrimino.get_rotated_points(direction);

        if self.board.do_points_fit(oriented_points.clone()) {
            self.active_tetrimino.rotate(direction);
            self.active_tetrimino_updated();

            true
        } else {
            let tetrimino = self.active_tetrimino.get_tetrimino().tetrimino_type;
            let wall_kicks = tetrimino
                .get_wall_kicks(self.active_tetrimino.orientation, direction);

            for wall_kick in wall_kicks.iter().cloned() {
                let translated = oriented_points
                    .iter()
                    .map(|p| *p + wall_kick)
                    .collect::<Vec<_>>();

                if self.board.do_points_fit(translated) {
                    self.active_tetrimino.translate(wall_kick);
                    self.active_tetrimino.rotate(direction);

                    return true;
                }
            }

            false
        }
    }
}
