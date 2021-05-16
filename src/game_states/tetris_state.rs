use macroquad::prelude::*;

use super::GameState;
use super::menu_state::MenuState;

use crate::drivers::*;
use crate::game_core::GameCore;
use crate::game_core::tetriminos::Tetrimino;
use crate::game_core::utils::point::Point;
use crate::ui::game_widgets::tetris_board::TetrisBoard;
use crate::ui::game_widgets::tetrimino_display::TetriminoDisplay;
use crate::ui::game_widgets::label::Label;
use crate::ui::game_widgets::widget::*;
use crate::ui::button::ButtonHandler;


const HOLD_DELAY: usize = 15;
const HOLD_RATE: usize = 3;

pub struct TetrisState {
    driver: Box<dyn Driver>,

    widgets: Vec<Box<dyn Widget>>,
    buttons: Vec<ButtonHandler<Self, ()>>,
    /* 
     * Whenever a new piece is added, the holdable buttons must be reset to
     * avoid accidentally fastfalling several tetriminos.
    */
    reset_button_holds: bool,

    transition_duration: usize,
    transition_elapsed: usize,
    transition: BoardTransition,
}

impl TetrisState {
    pub fn new(driver: Box<dyn Driver>) -> Box<Self> {
        let board_dimensions = Point((screen_height() * 0.8 * 0.5) as i32, 10 + (screen_height() * 0.8) as i32);
        let board_position = Point((screen_width() as i32 - board_dimensions.x()) / 2, (screen_height() as i32 - board_dimensions.y()) / 2);
        let tetris_board = TetrisBoard::new((board_position, board_position + board_dimensions));
        
        /*
         * Create all of the widgets for displaying the game.
         */
        let tetrimino_display_dimensions = Point(60, 60);
        let hold_position = board_position - Point::unit_x(tetrimino_display_dimensions.x() + 20) + Point(0, 20);
        let hold_display = TetriminoDisplay::new(
                (hold_position, hold_position + tetrimino_display_dimensions),
                driver.get_game_core(), 
                |core| core.get_held());

        let function_points: Vec<for <'a> fn(&'a GameCore) -> Option<&'a Tetrimino>> = vec![
            |core| Some(core.get_next_tetrimino(0)),
            |core| Some(core.get_next_tetrimino(1)),
            |core| Some(core.get_next_tetrimino(2)),
        ];
        let queue_display = (0..driver.get_game_core().get_tetrimino_queue_length())
            .map(|i| {
                let position = board_position + Point(board_dimensions.x() + 20, 80 * i as i32);
                TetriminoDisplay::new(
                    (position, position + tetrimino_display_dimensions),
                    driver.get_game_core(), 
                    function_points[i])
            })
            .collect::<Vec<_>>();
        
        let score_position = board_position + Point(board_dimensions.x(), 0) + Point(100, 20);
        let score_display = Label::new(
            score_position, 
            RED, 
            32.0, 
            |driver| format!("score: {}", driver.get_score()));
        let level_display = Label::new(
            score_position + Point::unit_y(40), 
            ORANGE, 
            32.0, 
            |driver| format!("level: {}", driver.get_level()));

        let mut widgets: Vec<Box<dyn Widget>> = Vec::new();
        widgets.push(Box::new(tetris_board));
        widgets.push(Box::new(hold_display));
        for d in queue_display.into_iter() {
            widgets.push(Box::new(d));
        }
        widgets.push(Box::new(score_display));
        widgets.push(Box::new(level_display));

        /*
         * Create all of the buttons.
         */
        let rotate_cc = ButtonHandler::pressable(KeyCode::A, |state: &mut TetrisState| { state.driver.rotate_counterclockwise(); });
        let rotate_c = ButtonHandler::pressable(KeyCode::D, |state: &mut TetrisState| { state.driver.rotate_clockwise(); });
        let hold = ButtonHandler::pressable(KeyCode::W, |state: &mut TetrisState| { state.driver.hold(); });

        let left = ButtonHandler::holdable(KeyCode::Left, HOLD_DELAY, HOLD_RATE, |state: &mut TetrisState| { state.driver.translate_left(); });
        let right = ButtonHandler::holdable(KeyCode::Right, HOLD_DELAY, HOLD_RATE, |state: &mut TetrisState| { state.driver.translate_right(); });
        let down = ButtonHandler::holdable(KeyCode::Down, HOLD_DELAY, HOLD_RATE, |state: &mut TetrisState| { 
            let (added, new_transition) = state.driver.fall();
            state.transition = new_transition;
            if added {
                state.reset_button_holds = true;
            }
        });
        let fastfall = ButtonHandler::pressable(KeyCode::Up, |state: &mut TetrisState| {                 
            let new_transition = state.driver.fastfall().1;
            state.transition = new_transition;
        });

        let buttons = vec![
            rotate_cc, rotate_c, hold,
            left, right, down, fastfall
        ];
    
        Box::new(Self {
            driver,

            widgets,
            buttons,
            reset_button_holds: false,

            transition: BoardTransition::new(),
            transition_duration: 10,
            transition_elapsed: 0,
        })
    }
}

impl GameState for TetrisState {
    fn next_frame(&mut self) -> (usize, Vec<Box<dyn GameState>>) {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::P) {
            return (0, vec![MenuState::new(Vec::new())])
        }

        if self.transition.is_inert() {
            self.transition = self.driver.next_frame();

            let mut buttons = std::mem::replace(&mut self.buttons, Vec::new());
            for button in buttons.iter_mut() {
                button.update(self);
            }
            self.buttons = buttons;
            if self.reset_button_holds {
                for button in self.buttons.iter_mut() {
                    button.reset_hold();
                }

                self.reset_button_holds = false;
            }
        } else {
            self.transition_elapsed += 1;
            if self.transition_elapsed > self.transition_duration {
                self.transition_elapsed = 0;
                let new_transition = self.driver.finish_transition(std::mem::replace(&mut self.transition, BoardTransition::new()));
                self.transition = new_transition;
            }
        }

        let widget_state = WidgetState {
            driver: self.driver.as_ref(),
            transition: &self.transition,
            transition_duration: self.transition_duration,
            transition_elapsed: self.transition_elapsed
        };

        for widget in self.widgets.iter() {
            widget.draw(widget_state)
        }

        return (0, Vec::new())
    }
}
