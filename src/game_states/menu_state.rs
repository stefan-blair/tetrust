use macroquad::prelude::*;
use futures::future::FutureExt;
use async_trait::async_trait;

use std::pin::Pin;

use super::*;
use crate::ui::button::ButtonHandler;


type FutureGameState<'a, 'b> = Pin<Box<dyn FutureExt<Output = Box<dyn GameState<'a> + 'a>> + 'b>>;

pub struct MenuOption<'a> {
    title: String,
    command: Box<dyn for<'b> Fn(&'b mut GameStateManager<'a>) -> FutureGameState<'a, 'b> + 'a>
}

impl<'a> MenuOption<'a> {
    pub fn new(title: String, command: impl for<'b> Fn(&'b mut GameStateManager<'a>) -> FutureGameState<'a, 'b> + 'a) -> Self {
        Self {
            title, command: Box::new(command)
        }
    }
}

pub struct MenuState<'a> {
    options: Vec<MenuOption<'a>>,
    buttons: Vec<ButtonHandler<Self, Option<usize>>>,
    selected_option: usize,
}

impl<'a> MenuState<'a> {
    pub fn new(options: Vec<MenuOption<'a>>) -> Self {
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
            Some(state.selected_option)
        });

        let buttons = vec![up_button, down_button, enter_button];

        Self {
            options,
            buttons,
            selected_option: 0
        }
    }
}

#[async_trait(?Send)]
impl<'a> GameState<'a> for MenuState<'a> {
    async fn run(mut self: Box<Self>, gamestate_manager: &mut GameStateManager<'a>) {
        loop {
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
                if let Some(index) = button.update(&mut self).flatten() {
                    let state_transition = (self.options[index].command)(gamestate_manager).await;
                    gamestate_manager.get_gamestate_stack().push(self);
                    gamestate_manager.get_gamestate_stack().push(state_transition);
                    return;
                }
            }
            self.buttons = buttons;

            next_frame().await;
        }
    }
}