use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

use crate::drivers::*;
use crate::game_core::GameCore;
use super::recording::*;


pub struct ReplayingDriver<'a> {
    wrapped: Box<dyn Driver + 'a>,
    current_frame: usize,
    actions: Vec<(usize, Action)>,
}

impl<'a> ReplayingDriver<'a> {
    pub fn new(wrapped: Box<dyn Driver + 'a>, source_file: &'static str) -> Self {
        let mut file = File::open(source_file).unwrap();
        let mut contents = vec![b'['];
        file.read_to_end(&mut contents).unwrap();
        contents.pop();
        contents.push(b']');

        // parse the actions from the recorder
        let mut actions: Vec<(usize, Action)> = serde_json::from_slice(&contents).unwrap();
        // reverse the array, because the replayer only cares about the next element (pop)
        actions.reverse();

        Self {
            wrapped,
            current_frame: 0,
            actions,
        }
    }
}

impl<'a> Driver for ReplayingDriver<'a> {
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
        let mut transitions = self.wrapped.next_frame();

        self.current_frame += 1;
        while let Some((frame, _)) = self.actions.last() {
            if *frame != self.current_frame {
                break;
            }

            let action = self.actions.pop().unwrap().1;
            match action {
                Action::TranslateLeft => { self.wrapped.translate_left(); }
                Action::TranslateRight => { self.wrapped.translate_right(); }
                Action::RotateClockwise => { self.wrapped.rotate_clockwise(); }
                Action::RotateCounterClockwise => { self.wrapped.rotate_counterclockwise(); }
                Action::Hold => self.wrapped.hold(),
                Action::Fastfall => {
                    transitions.add_to_transition(self.wrapped.fastfall().1);
                },
                Action::Fall => {
                    transitions.add_to_transition(self.wrapped.fall().1);
                },
            }
        }

        transitions
    }

    fn translate_left(&mut self) -> bool {
        if self.actions.is_empty() {
            self.wrapped.translate_left()
        } else {
            false
        }
    }
    fn translate_right(&mut self) -> bool {
        if self.actions.is_empty() {
            self.wrapped.translate_right()
        } else {
            false
        }
    }
    fn rotate_clockwise(&mut self) -> bool {
        if self.actions.is_empty() {
            self.wrapped.rotate_clockwise()
        } else {
            false
        }
    }
    fn rotate_counterclockwise(&mut self) -> bool {
        if self.actions.is_empty() {
            self.wrapped.rotate_counterclockwise()
        } else {
            false
        }
    }
    fn hold(&mut self) {
        if self.actions.is_empty() {
            self.wrapped.hold()
        }
    }

    fn fall(&mut self) -> (bool, BoardTransition) {
        if self.actions.is_empty() {
            self.wrapped.fall()
        } else {
            (false, BoardTransition::new())
        }
    }

    fn fastfall(&mut self) -> (i32, BoardTransition) {
        if self.actions.is_empty() {
            self.wrapped.fastfall()
        } else {
            (0, BoardTransition::new())
        }
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
