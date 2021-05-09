use macroquad::prelude::*;

use super::GameState;


pub struct MenuOption {
    title: String,
    command: Box<dyn Fn() -> Box<dyn GameState>>
}

impl MenuOption {
    pub fn new(title: String, command: Box<dyn Fn() -> Box<dyn GameState>>) -> Self {
        Self {
            title, command
        }
    }
}

pub struct MenuState {
    options: Vec<MenuOption>,
    selected_option: usize,
}

impl MenuState {
    pub fn new(options: Vec<MenuOption>) -> Self {
        Self {
            options,
            selected_option: 0
        }
    }
}

impl GameState for MenuState {
    fn next_frame(&mut self) -> (bool, Vec<Box<dyn GameState>>) {
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

        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            self.selected_option = self.selected_option.wrapping_sub(1) % self.options.len();
        }

        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            self.selected_option = (self.selected_option + 1) % self.options.len();
        }

        if is_key_pressed(KeyCode::Enter) {
            return (false, vec![(self.options[self.selected_option].command)()])
        }

        (false, Vec::new())
    }
}