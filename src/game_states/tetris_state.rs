use macroquad::prelude::*;

use super::GameState;
use super::menu_state::MenuState;

use crate::drivers::*;
use crate::game_core::utils::point::Point;
use crate::ui::game_widgets::tetris_board::TetrisBoard;
use crate::ui::game_widgets::tetrimino_display::TetriminoDisplay;
use crate::ui::game_widgets::widget::*;


pub struct TetrisState<'a> {
    driver: Box<dyn Driver<'a>>,
    widgets: Vec<Box<dyn Widget>>,
    transition_duration: usize,
    transition_elapsed: usize,
    transitions: Vec<BoardTransition>
}

impl<'a> TetrisState<'a> {
    pub fn new(driver: Box<dyn Driver<'a>>) -> Self {
        let tetris_board = TetrisBoard::new((Point(80, 10), Point(280, 410)));
        let hold_display = TetriminoDisplay::new(
                (Point(10, 40), Point(70, 100)),
                driver.get_game_core(), 
                |core| core.get_held());

        let queue_display = (0..driver.get_game_core().get_tetrimino_queue_length())
            .map(|i| TetriminoDisplay::new(
                (Point(300, 40 + 80 * i as i32), Point(360, 100 + 80 * i as i32)),
                driver.get_game_core(), 
                |core| Some(core.get_next_tetrimino(0))))
            .collect::<Vec<_>>();

        let mut widgets: Vec<Box<dyn Widget>> = Vec::new();
        widgets.push(Box::new(tetris_board));
        widgets.push(Box::new(hold_display));
        for d in queue_display.into_iter() {
            widgets.push(Box::new(d));
        }
    
        Self {
            driver,
            widgets,
            transitions: Vec::new(),
            transition_duration: 0,
            transition_elapsed: 0,
        }
    }
}

impl<'a> GameState for TetrisState<'a> {
    fn next_frame(&mut self) -> (bool, Vec<Box<dyn GameState>>) {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::P) {
            return (false, vec![Box::new(MenuState::new(Vec::new()))])
        }

        if self.transitions.is_empty() {
            self.transitions = self.driver.next_frame();
                        
            if is_key_pressed(KeyCode::A) {
                self.driver.rotate_counterclockwise();
            }
    
            if is_key_pressed(KeyCode::D) {
                self.driver.rotate_clockwise();
            }
    
            if is_key_pressed(KeyCode::W) {
                self.driver.get_game_core_mut().hold();
            }
    
            if is_key_pressed(KeyCode::Left) {
                self.driver.translate_left();
            }
            if is_key_pressed(KeyCode::Right) {
                self.driver.translate_right();
            }
            if is_key_pressed(KeyCode::Down) {
                let mut new_transitions = self.driver.fall();
                self.transitions.append(&mut new_transitions);
            }
            if is_key_pressed(KeyCode::Up) {
                let mut new_transitions = self.driver.fastfall();
                self.transitions.append(&mut new_transitions);
            }    
        } else {
            self.transition_elapsed += 1;
            if self.transition_elapsed > self.transition_duration {
                self.transition_elapsed = 0;
                let mut new_transitions = self.driver.finish_transition(self.transitions.remove(0));
                self.transitions.append(&mut new_transitions);
            }
        }

        let widget_state = WidgetState {
            driver: self.driver.as_ref(),
            transition: self.transitions.first(),
            transition_duration: self.transition_elapsed,
            transition_elapsed: self.transition_duration
        };

        for widget in self.widgets.iter() {
            widget.draw(widget_state)
        }

        return (false, Vec::new())
    }
}
