use rand::seq::SliceRandom;
use rand::RngCore;

use crate::game::board;
use crate::game::point::Point;
use crate::game::orientations::{Direction, Orientation};
use crate::game::tetriminos;

pub struct Engine<'a> {
    tetrimino_types: &'a Vec<tetriminos::Tetrimino>,
    active_tetrimino: tetriminos::ActiveTetrimino<'a>,
    // these usizes are indexes into the tetrimino_types vector
    held_tetrimino: Option<&'a tetriminos::Tetrimino>,

    tetrimino_queue: Vec<&'a tetriminos::Tetrimino>,
    next_tetrimino_index: usize,

    board: board::Board,

    rng: &'a mut dyn RngCore,
}

impl<'a> Engine<'a> {
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
            .with_position(Point(
                board.get_width() as i32 / 2,
                board.get_height() as i32,
            ));

        let tetrimino_queue = (0..queue_length)
            .map(|_| tetrimino_types.choose(rng).unwrap())
            .collect::<Vec<_>>();

        Self {
            tetrimino_types,
            active_tetrimino,
            held_tetrimino: None,
            tetrimino_queue,
            next_tetrimino_index: 0,
            board,
            rng,
        }
    }

    pub fn next_tetrimino(&mut self) {
        self.active_tetrimino = self.tetrimino_queue[self.next_tetrimino_index]
            .active_instance()
            .with_position(self.board.get_spawn_point());
        self.tetrimino_queue[self.next_tetrimino_index] = self.tetrimino_types
            .choose(self.rng)
            .unwrap();
        self.next_tetrimino_index += 1;
    }

    pub fn hold(&mut self) {
        let held_tetrimino = self.held_tetrimino;
        self.held_tetrimino = Some(self.active_tetrimino.get_tetrimino());
        
        match held_tetrimino {
            Some(held_tetrimino) => {
                self.active_tetrimino = held_tetrimino
                    .active_instance()
                    .with_position(self.board.get_spawn_point());
            }
            None => {
                self.next_tetrimino()
            }
        }
    }

    pub fn translate(&mut self, direction: Point) -> bool {
        let translated_tetrimino = self.active_tetrimino
            .translated(direction);
            
        if self.board.does_tetrimino_fit(translated_tetrimino) {
            self.active_tetrimino = translated_tetrimino;

            true
        } else {
            false
        }
    }

    pub fn translate_left(&mut self) -> bool {
        self.translate(Point(-1, 0))
    }

    pub fn translate_right(&mut self) -> bool {
        self.translate(Point(1, 0))
    }

    pub fn fall(&mut self) -> bool {
        // if the piece can fall no further, then place it and get the next piece
        if !self.translate(Point(0, -1)) {
            self.board.add_tetrimino(self.active_tetrimino);
            self.next_tetrimino();
            false
        } else {
            true
        }
    }

    pub fn rotate(&mut self, direction: Direction) -> bool {
        let oriented_tetrimino = self.active_tetrimino.rotated(direction);

        if self.board.does_tetrimino_fit(oriented_tetrimino) {
            self.active_tetrimino = oriented_tetrimino;

            true
        } else {
            let wall_kicks = self.active_tetrimino
                .get_tetrimino()
                .get_wall_kicks(self.active_tetrimino.orientation, direction);

            for wall_kick in wall_kicks.iter() {
                let translated_tetrimino = oriented_tetrimino
                    .translated(*wall_kick);
                if self.board.does_tetrimino_fit(translated_tetrimino) {
                    self.active_tetrimino = translated_tetrimino;

                    return true
                }
            }

            false
        }
    }

    pub fn rotate_clockwise(&mut self) -> bool {
        self.rotate(Direction::Clockwise)
    }

    pub fn rotate_counterclockwise(&mut self) -> bool {
        self.rotate(Direction::CounterClockwise)
    }
}
