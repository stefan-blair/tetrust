use std::fs::File;
use std::io::prelude::*;

use crate::drivers::*;
use super::recording::*;


pub struct ReplayingDriver<'a> {
    wrapped: Box<dyn Driver + 'a>,
    current_frame: usize,
    actions: Vec<(usize, Action)>,
}

impl<'a> ReplayingDriver<'a> {
    pub fn new(wrapped: Box<dyn Driver + 'a>, source_file: &str) -> Self {
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
    fn get_driver_core(&self) -> &DriverCore {
        self.wrapped.get_driver_core()
    }

    fn get_driver_core_mut(&mut self) -> &mut DriverCore {
        self.wrapped.get_driver_core_mut()
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
                    transitions.add_from_transition(self.wrapped.fastfall());
                },
                Action::Fall => {
                    transitions.add_from_transition(self.wrapped.fall());
                },
            }
        }

        transitions
    }

    fn translate_left(&mut self){
        if self.actions.is_empty() {
            self.wrapped.translate_left()
        }
    }
    fn translate_right(&mut self){
        if self.actions.is_empty() {
            self.wrapped.translate_right()
        }
    }
    fn rotate_clockwise(&mut self){
        if self.actions.is_empty() {
            self.wrapped.rotate_clockwise()
        }
    }
    fn rotate_counterclockwise(&mut self){
        if self.actions.is_empty() {
            self.wrapped.rotate_counterclockwise()
        }
    }
    fn hold(&mut self) {
        if self.actions.is_empty() {
            self.wrapped.hold()
        }
    }

    fn fall(&mut self) -> BoardTransition {
        if self.actions.is_empty() {
            self.wrapped.fall()
        } else {
            BoardTransition::new()
        }
    }

    fn fastfall(&mut self) -> BoardTransition {
        if self.actions.is_empty() {
            self.wrapped.fastfall()
        } else {
            BoardTransition::new()
        }
    }

    fn finish_transition(&mut self, transition: BoardTransition) -> BoardTransition {
        self.wrapped.finish_transition(transition)
    }
}
