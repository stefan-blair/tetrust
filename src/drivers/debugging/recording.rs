use std::fs::{OpenOptions, remove_file};
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

use crate::drivers::*;
use crate::game_core::GameCore;


pub struct RecordingGenerator {
    tetrimino_type_chooser: utils::tetrimino_chooser::TetriminoChooser
}

impl RecordingGenerator {
    pub fn new(tetrimino_types: &'static [TetriminoType]) -> Box<Self> {
        Box::new(Self {
            tetrimino_type_chooser: utils::tetrimino_chooser::TetriminoChooser::new(tetrimino_types)
                .with_seed(Vec::new())
        })
    }
}

impl TetriminoGenerator for RecordingGenerator {
    fn next(&mut self) -> Tetrimino {
        let (index, tetrimino_type) = self.tetrimino_type_chooser.choose_tetrimino_type();
        let values = vec![index as u32; 4];
        tetrimino_type.instance(values)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Action {
    TranslateLeft,
    TranslateRight,
    RotateClockwise,
    RotateCounterClockwise,
    Hold,
    Fastfall,
    Fall,
}

pub struct RecordingDriver<'a> {
    wrapped: Box<dyn Driver + 'a>,
    current_frame: usize,
    actions: Vec<(usize, Action)>,
    destination_file: &'static str,
}

impl<'a> RecordingDriver<'a> {
    pub fn new(wrapped: Box<dyn Driver + 'a>, destination_file: &'static str) -> Self {
        remove_file(destination_file).ok();
        Self {
            wrapped,
            current_frame: 0,
            actions: Vec::new(),
            destination_file
        }
    }

    pub fn push_action(&mut self, action: Action) {
        let current_frame = self.current_frame;
        self.actions.push((current_frame, action));
        
        let serialized = serde_json::to_vec(&(current_frame, action)).unwrap();

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(self.destination_file)
            .unwrap();
        file.write_all(&serialized).unwrap();
        file.write_all(b",").unwrap();
    }
}

impl<'a> Driver for RecordingDriver<'a> {
    fn get_generator(tetrimino_types: &'static [TetriminoType]) -> Box<dyn TetriminoGenerator> where Self: Sized{
        RecordingGenerator::new(tetrimino_types)
    }

    fn get_game_core(&self) -> &GameCore {
        self.wrapped.get_game_core()
    }
    fn get_game_core_mut(&mut self) -> &mut GameCore {
        self.wrapped.get_game_core_mut()
    }
    fn get_score(&self) -> usize {
        self.wrapped.get_score()
    }
    fn get_level(&self) -> usize {
        self.wrapped.get_level()
    }
    fn next_frame(&mut self) -> BoardTransition {
        self.current_frame += 1;

        self.wrapped.next_frame()
    }

    fn translate_left(&mut self) -> bool {
        self.push_action(Action::TranslateLeft);

        self.wrapped.translate_left()
    }
    fn translate_right(&mut self) -> bool {
        self.push_action(Action::TranslateRight);

        self.wrapped.translate_right()
    }
    fn rotate_clockwise(&mut self) -> bool {
        self.push_action(Action::RotateClockwise);

        self.wrapped.rotate_clockwise()
    }
    fn rotate_counterclockwise(&mut self) -> bool {
        self.push_action(Action::RotateCounterClockwise);

        self.wrapped.rotate_counterclockwise()
    }
    fn hold(&mut self) {
        self.push_action(Action::Hold);

        self.wrapped.hold()
    }

    fn fall(&mut self) -> (bool, BoardTransition) {
        self.push_action(Action::Fall);
        
        self.wrapped.fall()
    }

    fn fastfall(&mut self) -> (i32, BoardTransition) {
        self.push_action(Action::Fastfall);

        self.wrapped.fastfall()
    }

    fn rows_cleared(&mut self, rows: Vec<i32>) -> BoardTransition {
        self.wrapped.rows_cleared(rows)
    }

    fn points_cleared(&mut self, points: Vec<Point>) -> BoardTransition {
        self.wrapped.points_cleared(points)
    }

    fn points_fell(&mut self, points: Vec<(Point, i32)>, full_rows: Vec<i32>) -> BoardTransition {
        self.wrapped.points_fell(points, full_rows)
    }

    fn finish_transition(&mut self, transition: BoardTransition) -> BoardTransition {
        self.wrapped.finish_transition(transition)
    }
}
