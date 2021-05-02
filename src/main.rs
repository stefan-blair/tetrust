mod game;
use game::point::Point;

fn display_on_board(points: &[Point]) {
    let block = String::from_utf16(&[0x2588]).unwrap().pop().unwrap();

    println!("----------------------");

    for y in 0..20 {
        print!("|");
        for x in 0..10 {
            if points.iter().find(|&&p| p == Point(x, 19 - y)).is_some() {
                print!("{}{}", block, block);
            } else {
                print!("  ");
            }
        }
        println!("| {}", 19 - y);
    }

    println!("----------------------");
}

// fn main() {
//     let tetrimino_types = game::defaults::tetriminos::tetrimino_types();
//     for tetrimino_type in tetrimino_types.iter() {
//         let mut instance = tetrimino_type.instantiate(Point(0, 10));
//         display_on_board(&instance.get_points());
//         instance.rotate_clockwise();
//         display_on_board(&instance.get_points());
//         instance.rotate_clockwise();
//         display_on_board(&instance.get_points());
//         instance.rotate_clockwise();
//         display_on_board(&instance.get_points());
//     }
// }

use macroquad::prelude::*;

fn draw_shapes() {
    draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
}

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(RED);

        draw_shapes();

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
