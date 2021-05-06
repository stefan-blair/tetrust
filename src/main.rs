use ::rand::thread_rng;
use macroquad::prelude::*;

mod drivers;
mod game_core;
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
    let core = game_core::GameCore::new(&tetrimino_types, board, queue_length, &mut rng);

    let mut driver = drivers::Driver::new(core, TEST_GRAVITY, 0.5);

    let tetris_board = ui::tetris_board::TetrisBoard;
    let hold_display =
        ui::tetrimino_display::TetriminoDisplay::new(driver.get_game_core(), |core| {
            core.get_held()
        });
    let queue_display = vec![
        ui::tetrimino_display::TetriminoDisplay::new(driver.get_game_core(), |core| {
            Some(core.get_next_tetrimino(0))
        }),
        ui::tetrimino_display::TetriminoDisplay::new(driver.get_game_core(), |core| {
            Some(core.get_next_tetrimino(1))
        }),
        ui::tetrimino_display::TetriminoDisplay::new(driver.get_game_core(), |core| {
            Some(core.get_next_tetrimino(2))
        }),
    ];

    loop {
        // add gravity to the engine and a time update function
        // get_frame_time()
        // make a driver, which takes an engine and does actual game shit with it

        // get_frame_time()

        clear_background(BLUE);

        driver.next_frame();

        if is_key_pressed(KeyCode::Space) {
            driver.get_game_core_mut().next_tetrimino()
        }

        if is_key_pressed(KeyCode::A) {
            driver.get_game_core_mut().rotate_counterclockwise();
        }

        if is_key_pressed(KeyCode::D) {
            driver.get_game_core_mut().rotate_clockwise();
        }

        if is_key_pressed(KeyCode::W) {
            driver.get_game_core_mut().hold();
        }

        if is_key_pressed(KeyCode::Left) {
            driver.get_game_core_mut().translate_left();
        }
        if is_key_pressed(KeyCode::Right) {
            driver.get_game_core_mut().translate_right();
        }
        if is_key_pressed(KeyCode::Down) {
            driver.get_game_core_mut().fall();
        }
        if is_key_pressed(KeyCode::Up) {
            driver.get_game_core_mut().fastfall();
        }

        tetris_board.draw(&driver, (Point(80, 10), Point(280, 410)));
        hold_display.draw(&driver, (Point(10, 40), Point(70, 100)));
        for (i, display) in queue_display.iter().enumerate() {
            display.draw(&driver, (Point(300, 40 + 80 * i as i32), Point(360, 100 + 80 * i as i32)));
        }

        next_frame().await

        /*
            The game could be optimized such that all of the logic occurs on
            one thread, and sends instructions on how to render to another thread
        */
    }
}
