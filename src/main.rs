use macroquad::prelude::*;

mod drivers;
mod game_core;
mod game_states;
mod ui;

use drivers::Driver;
use game_core::utils::point::Point;
use game_core::defaults;
use game_states::GameState;
use game_states::menu_state::*;
use game_states::tetris_state::*;


const TEST_GRAVITY: &[usize] = &[59];

#[macroquad::main("TetRust")]
async fn main() {
    let tetrimino_types = defaults::tetriminos::tetrimino_types();
    let width = defaults::dimensions::CELL_WIDTH;
    let height = defaults::dimensions::CELL_HEIGHT;
    let queue_length = game_core::defaults::settings::QUEUE_LENGTH;

    let mut menu_state = MenuState::new(vec![
        MenuOption::new("regular".to_string(), Box::new(|| Box::new(MenuState::new(vec![])))),
        // MenuOption::new("sticky".to_string(), Box::new(move || {
        //     let board = game_core::board::Board::new(width, height);
        //     // initialize game engine
        //     let core = game_core::GameCore::new(tetrimino_types_reference, board, queue_length);
        //     let driver = Box::new(drivers::sticky_driver::StickyDriver::new(core, TEST_GRAVITY, 0.5));
    
        //     Box::new(TetrisState::new(driver))
        // })),
        MenuOption::new("cascade".to_string(), Box::new(|| Box::new(MenuState::new(vec![])))),
        MenuOption::new("options".to_string(), Box::new(|| Box::new(MenuState::new(vec![])))),
    ]);

    loop {
        menu_state.next_frame();

        next_frame().await
    }
}
