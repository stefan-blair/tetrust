use ::rand::thread_rng;
use macroquad::prelude::*;

mod drivers;
mod game_core;
mod ui;

use drivers::Driver;
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
    let mut board = game_core::board::Board::new(width, height);

    let points = vec![
        // (1, 4, 1),
        // (1, 5, 1),
        // (1, 6, 1),
        // (1, 7, 1),
        // (1, 8, 1),
        // (1, 9, 1),
        // (1, 10, 1),
        // (1, 11, 1),
        // (1, 12, 1),
        // (1, 13, 1),
        // (1, 14, 1),
        // (1, 15, 1),
        // (2, 15, 1),
        // (2, 4, 1),
        // (3, 4, 1),
        // (3, 5, 1),
        // (3, 3, 1),
        // (3, 6, 1),
        // (3, 2, 1),
        // (4, 6, 1),
        // (4, 2, 1),


        // (4, 4, 2),
        // (5, 4, 2),
        // (5, 5, 2),
        // (5, 6, 2),
        // (5, 7, 2),
        // (5, 8, 2),
        // (4, 8, 2),
        // (5, 9, 2),
        // (5, 10, 2),
        // (5, 11, 2),
        // (5, 12, 2),
        // (4, 12, 2),

        // (4, 10, 3),
        // (3, 10, 3),
        // (2, 10, 3),
        // (2, 9, 3),
        // (2, 8, 3),
    ];

    for (x, y, value) in points {
        board.fill_point(Point(x, y), value);
    }

    let queue_length = game_core::defaults::settings::QUEUE_LENGTH;
    let mut rng = thread_rng();
    // initialize game engine
    let core = game_core::GameCore::new(&tetrimino_types, board, queue_length, &mut rng);

    let mut driver = drivers::sticky_driver::StickyDriver::new(core, TEST_GRAVITY, 0.5);

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

    let mut pause = false;

    loop {
        // add gravity to the engine and a time update function
        // get_frame_time()
        // make a driver, which takes an engine and does actual game shit with it

        // get_frame_time()

        clear_background(BLUE);

        if is_key_pressed(KeyCode::P) {
            if pause {
                pause = false;
            } else {
                pause = true
            }
        }

        if !pause {
            driver.next_frame();
        }
        
        if is_key_pressed(KeyCode::Space) {
            let board = driver.get_game_core().get_board();
            let bottom_points = (0..board.get_width()).map(|x| Point(x as i32, 0)).filter(|p| board.is_point_filled(*p)).collect::<Vec<_>>();
            println!("bottom_points == {:?}", bottom_points);
            driver.calculate_sticky_falls(bottom_points);
        }
        
        if is_key_pressed(KeyCode::A) {
            driver.rotate_counterclockwise();
        }

        if is_key_pressed(KeyCode::D) {
            driver.rotate_clockwise();
        }

        if is_key_pressed(KeyCode::W) {
            driver.get_game_core_mut().hold();
        }

        if is_key_pressed(KeyCode::Left) {
            driver.translate_left();
        }
        if is_key_pressed(KeyCode::Right) {
            driver.translate_right();
        }
        if is_key_pressed(KeyCode::Down) {
            driver.fall();
        }
        if is_key_pressed(KeyCode::Up) {
            driver.fastfall();
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
