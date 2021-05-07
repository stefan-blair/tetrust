use rand::seq::SliceRandom;
use rand::RngCore;

pub mod board;
pub mod defaults;
pub mod tetriminos;
pub mod utils;

use utils::orientations::Direction;
use utils::point::Point;

pub struct GameCore<'a> {
    tetrimino_types: &'a Vec<tetriminos::Tetrimino>,
    active_tetrimino: tetriminos::ActiveTetrimino<'a>,
    ghost_tetrimino: tetriminos::ActiveTetrimino<'a>,
    held_tetrimino: Option<&'a tetriminos::Tetrimino>,

    tetrimino_queue: Vec<&'a tetriminos::Tetrimino>,
    next_tetrimino_index: usize,

    board: board::Board,

    rng: &'a mut dyn RngCore,
}

impl<'a> GameCore<'a> {
    pub fn new(
        tetrimino_types: &'a Vec<tetriminos::Tetrimino>,
        board: board::Board,
        queue_length: usize,
        rng: &'a mut dyn RngCore,
    ) -> Self {
        let active_tetrimino = tetrimino_types
            .choose(rng)
            .unwrap()
            .active_instance()
            .with_position(board.get_spawn_point());

        let tetrimino_queue = (0..queue_length)
            .map(|_| tetrimino_types.choose(rng).unwrap())
            .collect::<Vec<_>>();

        Self {
            tetrimino_types,
            active_tetrimino,
            ghost_tetrimino: active_tetrimino,
            held_tetrimino: None,
            tetrimino_queue,
            next_tetrimino_index: 0,
            board,
            rng,
        }
    }

    pub fn get_board(&self) -> &board::Board {
        &self.board
    }

    pub fn get_active_tetrimino(&self) -> &tetriminos::ActiveTetrimino {
        &self.active_tetrimino
    }

    pub fn get_ghost_tetriminio(&self) -> &tetriminos::ActiveTetrimino {
        &self.ghost_tetrimino
    }

    pub fn get_next_tetrimino(&self, index: usize) -> &tetriminos::Tetrimino {
        &self.tetrimino_queue[(self.next_tetrimino_index + index) % self.tetrimino_queue.len()]
    }

    pub fn get_tetrimino_types(&self) -> &[tetriminos::Tetrimino] {
        &self.tetrimino_types
    }

    pub fn set_active_tetrimino(&mut self, active_tetrimino: tetriminos::ActiveTetrimino<'a>) {
        self.active_tetrimino = active_tetrimino;
        self.active_tetrimino_updated()
    }

    pub fn next_tetrimino(&mut self) {
        self.set_active_tetrimino(
            self.tetrimino_queue[self.next_tetrimino_index]
                .active_instance()
                .with_position(self.board.get_spawn_point()),
        );
        self.tetrimino_queue[self.next_tetrimino_index] =
            self.tetrimino_types.choose(self.rng).unwrap();
        self.next_tetrimino_index = (self.next_tetrimino_index + 1) % self.tetrimino_queue.len();
    }

    pub fn active_tetrimino_updated(&mut self) {
        let ghost_translation = self.board.first_collision(self.active_tetrimino);
        self.ghost_tetrimino = self.active_tetrimino.translated(ghost_translation);
    }

    pub fn hold(&mut self) {
        let held_tetrimino = self.held_tetrimino;
        self.held_tetrimino = Some(self.active_tetrimino.get_tetrimino());

        match held_tetrimino {
            Some(held_tetrimino) => {
                self.set_active_tetrimino(
                    held_tetrimino
                        .active_instance()
                        .with_position(self.board.get_spawn_point()),
                );
            }
            None => self.next_tetrimino(),
        }
    }

    pub fn get_held(&self) -> Option<&tetriminos::Tetrimino> {
        self.held_tetrimino
    }

    pub fn translate(&mut self, direction: Point) -> bool {
        let translated_tetrimino = self.active_tetrimino.translated(direction);

        if self.board.does_tetrimino_fit(translated_tetrimino) {
            self.set_active_tetrimino(translated_tetrimino);

            true
        } else {
            false
        }
    }

    pub fn add_tetrimino(&mut self) -> Option<Vec<i32>> {
        let rows = self.board.add_tetrimino(self.active_tetrimino);
        self.next_tetrimino();

        rows
    }

    pub fn fall(&mut self) -> (bool, Option<Vec<i32>>) {
        // if the piece can fall no further, then place it and get the next piece
        if !self.translate(Point(0, -1)) {
            (false, self.add_tetrimino())
        } else {
            (true, None)
        }
    }

    pub fn fastfall(&mut self) -> Option<Vec<i32>> {
        let translation = self.board.first_collision(self.active_tetrimino);
        self.active_tetrimino = self.active_tetrimino.translated(translation);
        self.add_tetrimino()
    }

    pub fn clear_rows(&mut self, rows: Vec<i32>) {
        self.board.clear_rows(rows)
    }

    pub fn clear_points(&mut self, points: Vec<Point>) {
        self.board.clear_points(points)
    }

    // could use a hashmap instead, but these are such small amounts of data that the overhead would likely be too much
    pub fn translate_falling_points(&mut self, point_drops: Vec<(Point, i32)>) {
        self.board.translate_falling_points(point_drops)
    }

    pub fn rotate(&mut self, direction: Direction) -> bool {
        let oriented_tetrimino = self.active_tetrimino.rotated(direction);

        if self.board.does_tetrimino_fit(oriented_tetrimino) {
            self.set_active_tetrimino(oriented_tetrimino);

            true
        } else {
            let wall_kicks = self
                .active_tetrimino
                .get_tetrimino()
                .get_wall_kicks(self.active_tetrimino.orientation, direction);

            for wall_kick in wall_kicks.iter() {
                let translated_tetrimino = oriented_tetrimino.translated(*wall_kick);
                if self.board.does_tetrimino_fit(translated_tetrimino) {
                    self.set_active_tetrimino(translated_tetrimino);

                    return true;
                }
            }

            false
        }
    }
}
