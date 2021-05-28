use std::fs::{OpenOptions, remove_file};
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

use crate::drivers::*;


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
    destination_file: String,
}

impl<'a> RecordingDriver<'a> {
    pub fn new(wrapped: Box<dyn Driver + 'a>, destination_file: String) -> Self {
        remove_file(&destination_file).ok();
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
            .open(&self.destination_file)
            .unwrap();
        file.write_all(&serialized).unwrap();
        file.write_all(b",").unwrap();
    }
}

impl<'a> Driver for RecordingDriver<'a> {
    fn get_driver_core(&self) -> &DriverCore {
        self.wrapped.get_driver_core()
    }

    fn get_driver_core_mut(&mut self) -> &mut DriverCore {
        self.wrapped.get_driver_core_mut()
    }

    fn next_frame(&mut self) -> BoardTransition {
        self.current_frame += 1;

        self.wrapped.next_frame()
    }

    fn translate_left(&mut self) {
        self.push_action(Action::TranslateLeft);

        self.wrapped.translate_left()
    }
    fn translate_right(&mut self) {
        self.push_action(Action::TranslateRight);

        self.wrapped.translate_right()
    }
    fn rotate_clockwise(&mut self) {
        self.push_action(Action::RotateClockwise);

        self.wrapped.rotate_clockwise()
    }
    fn rotate_counterclockwise(&mut self) {
        self.push_action(Action::RotateCounterClockwise);

        self.wrapped.rotate_counterclockwise()
    }
    fn hold(&mut self) {
        self.push_action(Action::Hold);

        self.wrapped.hold()
    }

    fn fall(&mut self) -> BoardTransition {
        self.push_action(Action::Fall);
        
        self.wrapped.fall()
    }

    fn fastfall(&mut self) -> BoardTransition {
        self.push_action(Action::Fastfall);

        self.wrapped.fastfall()
    }

    fn finish_transition(&mut self, transition: BoardTransition) -> BoardTransition {
        self.wrapped.finish_transition(transition)
    }
}
