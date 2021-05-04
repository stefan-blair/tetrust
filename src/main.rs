use ::rand::thread_rng;
use macroquad::prelude::*;

mod game_core;
mod drivers;
mod ui;

use game_core::utils::point::Point;
use ui::widget::Widget;

// fn display_on_board(points: &[Point]) {
//     let block = String::from_utf16(&[0x2588]).unwrap().pop().unwrap();

//     println!("----------------------");

//     for y in 0..20 {
//         print!("|");
//         for x in 0..10 {
//             if points.iter().find(|&&p| p == Point(x, 19 - y)).is_some() {
//                 print!("{}{}", block, block);
//             } else {
//                 print!("  ");
//             }
//         }
//         println!("| {}", 19 - y);
//     }

//     println!("----------------------");
// }

const TEST_GRAVITY: &[usize] = &[59];

#[macroquad::main("TetRust")]
async fn main() {
    let tetrimino_types = game_core::defaults::tetriminos::tetrimino_types();
    let width = game_core::defaults::dimensions::CELL_WIDTH;
    let height = game_core::defaults::dimensions::CELL_HEIGHT;
    let board = game_core::board::Board::new(width, height);
    let queue_length = game_core::defaults::settings::QUEUE_LENGTH;
    let mut rng = thread_rng();
    // initialize game engine
    let core = game_core::GameCore::new(
        &tetrimino_types, 
        board, 
        queue_length, 
        &mut rng);


    let tetris_board = ui::tetris_board::TetrisBoard;
    let mut driver = drivers::Driver::new(core, TEST_GRAVITY, 0.5);

    loop {

        // add gravity to the engine and a time update function
        // get_frame_time()
        // make a driver, which takes an engine and does actual game shit with it

        // get_frame_time()

        driver.next_frame();

        if is_key_pressed(KeyCode::Space) {
            driver.core.next_tetrimino()
        }
        
        if is_key_pressed(KeyCode::A) {
            driver.core.rotate_counterclockwise();
        }
        
        if is_key_pressed(KeyCode::D) {
            driver.core.rotate_clockwise();
        }
        
        if is_key_pressed(KeyCode::Left) {
            driver.core.translate_left();
        }
        if is_key_pressed(KeyCode::Right) {
            driver.core.translate_right();
        }
        if is_key_pressed(KeyCode::Down) {
            driver.core.fall();
        }
        if is_key_pressed(KeyCode::Up) {
            driver.core.fastfall();
        }
        
        tetris_board.draw(&driver.core, (Point(10, 10), Point(210, 410)));

        next_frame().await

        /*
            The game could be optimized such that all of the logic occurs on
            one thread, and sends instructions on how to render to another thread
        */
    }
}
