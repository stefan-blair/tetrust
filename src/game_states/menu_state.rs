use macroquad::prelude::*;
use futures::future::FutureExt;

use super::GameState;
use crate::ui::button::ButtonHandler;


type FutureGameState = Box<dyn FutureExt<Output = Box<dyn GameState>>>;

pub struct MenuOption {
    title: String,
    command: Box<dyn Fn() -> FutureGameState>
}

impl MenuOption {
    pub fn new(title: String, command: Box<dyn Fn() -> FutureGameState>) -> Self {
        Self {
            title, command
        }
    }
}

pub struct MenuState {
    options: Vec<MenuOption>,
    buttons: Vec<ButtonHandler<Self, Option<(usize, Vec<FutureGameState>)>>>,
    selected_option: usize,
}

impl MenuState {
    pub fn new(options: Vec<MenuOption>) -> Box<Self> {
        let up_button = ButtonHandler::holdable(KeyCode::Up, 20, 2, |state: &mut Self| {
            if state.selected_option == 0 {
                state.selected_option = state.options.len() - 1
            } else {
                state.selected_option -= 1
            }
            None
        });

        let down_button = ButtonHandler::holdable(KeyCode::Down, 20, 2, |state: &mut Self| {
            state.selected_option = (state.selected_option + 1) % state.options.len();
            None
        });

        let enter_button = ButtonHandler::pressable(KeyCode::Enter, |state: &mut Self| {
            Some((0, vec![(state.options[state.selected_option].command)()]))
        });

        let buttons = vec![up_button, down_button, enter_button];

        Box::new(Self {
            options,
            buttons,
            selected_option: 0
        })
    }
}

impl GameState for MenuState {
    fn next_frame(&mut self) -> (usize, Vec<Box<dyn GameState>>) {
        clear_background(BLACK);

        // draw tagline
        const TAGLINE: &str = "TetRust!";
        const FONT_SIZE: u16 = 128;
        let dim = measure_text(TAGLINE, None, FONT_SIZE, 1.0);
        let x_pos = (screen_width() - dim.width) / 2.0;
        let y_pos = dim.height + dim.offset_y + 50.0;
        draw_text(TAGLINE, x_pos, y_pos, FONT_SIZE as f32, RED);

        let colors = &[
            BLUE,
            GREEN,
            YELLOW,
            ORANGE,
            MAGENTA,
            SKYBLUE,
            RED,
        ];

        const DEFAULT_OPTION_FONT: f32 = 32.0;
        let dim = measure_text("A", None, DEFAULT_OPTION_FONT as u16, 1.0);
        let option_height = dim.height + dim.offset_y;
        for (i, option) in self.options.iter().enumerate() {
            let mut font_size = DEFAULT_OPTION_FONT;
            let mut color = colors[i];
            if self.selected_option == i {
                font_size *= 1.25;
            } else {
                color.a = 0.8;
            }

            let dim = measure_text(&option.title, None, font_size as u16, 1.0);
            let x_pos = (screen_width() - dim.width) / 2.0;
            let y_pos = (screen_height() / 2.0) + option_height * i as f32;
            draw_text(&option.title, x_pos, y_pos, font_size as f32, color);
        }

        let mut buttons = std::mem::replace(&mut self.buttons, Vec::new());
        for button in buttons.iter_mut() {
            if let Some(state_transition) = button.update(self).flatten() {
                return state_transition
            }
        }
        self.buttons = buttons;

        (0, Vec::new())
    }
}