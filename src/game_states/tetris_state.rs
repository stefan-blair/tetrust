use macroquad::prelude::*;
use async_trait::async_trait;

use super::*;
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
use crate::ui::rendering::*;
use crate::ui::utils::board_transition_progress::BoardTransitionsProgress;


const HOLD_DELAY: usize = 15;
const HOLD_RATE: usize = 3;
const FASTFALL_HOLD: usize = 10;

pub struct TetrisState {
    driver: Box<dyn Driver>,

    render_manager: RenderManager,
    widgets: Vec<Box<dyn Widget>>,
    buttons: Vec<ButtonHandler<Self, ()>>,
    /* 
     * Whenever a new piece is added, the holdable buttons must be reset to
     * avoid accidentally fastfalling several tetriminos.
    */
    reset_button_holds: bool,

    /*
     * Whenever a piece falls naturally, fastfall cannot be pressed for a few 
     * frames to avoid an accidental double drop.
     */
    fastfall_delay: (usize, usize),

    transition_durations: BoardTransitionsProgress,
    transition_progress: BoardTransitionsProgress,
    transition: BoardTransition,
}

impl TetrisState {
    pub fn new(driver: Box<dyn Driver>, render_manager: RenderManager) -> Self {
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

        let function_points: Vec<for <'b> fn(&'b GameCore) -> Option<&'b Tetrimino>> = vec![
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
        let down = ButtonHandler::pressable(KeyCode::Down, |state: &mut TetrisState| { 
            state.driver.start_fastfalling();
        }).with_release_action(|state: &mut TetrisState| {
            state.driver.stop_fastfalling();
        });
        let fastfall = ButtonHandler::pressable(KeyCode::Up, |state: &mut TetrisState| {
            if state.fastfall_delay.0 == 0 {
                let new_transition = state.driver.fastfall();
                state.set_transition(new_transition);
            }
        });

        let buttons = vec![
            rotate_cc, rotate_c, hold,
            left, right, down, fastfall
        ];
    
        Self {
            driver,
    
            render_manager,
            widgets,
            buttons,
            reset_button_holds: false,

            fastfall_delay: (0, FASTFALL_HOLD),

            transition: BoardTransition::new(),
            transition_durations: BoardTransitionsProgress::new(),
            transition_progress: BoardTransitionsProgress::new()
        }
    }

    fn set_transition(&mut self, transition: BoardTransition) {
        self.transition = transition;
        self.transition_progress = self.transition_durations.with_board_transition(&self.transition);
    }
}

#[async_trait(?Send)]
impl<'a> GameState<'a> for TetrisState {
    async fn run(mut self: Box<Self>, gamestate_manager: &mut GameStateManager) {
        loop {
            clear_background(BLACK);

            if is_key_pressed(KeyCode::P) {
                gamestate_manager.get_gamestate_stack().push(self);
                gamestate_manager.get_gamestate_stack().push(MenuState::new(Vec::new()).await.boxed());
                return;
            }

            // update the fastfall delay
            if self.fastfall_delay.0 > 0 {
                self.fastfall_delay.0 -= 1;
            }

            if self.transition.is_inert() {
                let new_transition = self.driver.next_frame(); 
                self.set_transition(new_transition);

                if self.transition.get_points_added().is_some() {
                    self.fastfall_delay.0 = self.fastfall_delay.1;
                }
                
                let mut buttons = std::mem::replace(&mut self.buttons, Vec::new());
                for button in buttons.iter_mut() {
                    button.update(self.as_mut());
                }
                self.buttons = buttons;
                if self.reset_button_holds {
                    for button in self.buttons.iter_mut() {
                        button.reset_hold();
                    }
    
                    self.reset_button_holds = false;
                }
            } else {
                self.transition_progress.next_frame();
                if self.transition_progress.is_complete() {
                    let new_transition = self.driver.finish_transition(std::mem::replace(&mut self.transition, BoardTransition::new()));
                    self.set_transition(new_transition);
                }
            }

            // make sure all arrays are sorted and deduped
            self.transition.compress();
    
            let widget_state = WidgetState {
                driver: self.driver.as_ref(),
                transition: &self.transition,
                transition_progress: self.transition_progress,
            };
    
            for widget in self.widgets.iter_mut() {
                widget.draw(widget_state, self.render_manager.get_rendering_state(widget_state));
            }
    
            next_frame().await;
        }
    }
}
