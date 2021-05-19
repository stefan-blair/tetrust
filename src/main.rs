#![feature(const_fn_floating_point_arithmetic)]

use macroquad::prelude::*;

#[macro_use]
mod game_core;
mod drivers;
mod game_states;
mod ui;

use drivers::*;

use game_states::GameState;
use game_states::menu_state::*;
use game_states::tetris_state::*;

use classic_driver::ClassicDriver;
use cascade_driver::CascadeDriver;
use sticky_driver::StickyDriver;
use fusion_driver::FusionDriver;

use ui::rendering::basic_tileset_renderer::BasicTilesetRenderManager;


#[macroquad::main("TetRust")]
async fn main() {
    let basic_tileset_renderer = Box::new(
        BasicTilesetRenderManager::new(
            "res/basic_tilemap.png", 
            "res/basic_tilemap_info.json").await);
    let render_2 = basic_tileset_renderer.clone();
    let render_3 = basic_tileset_renderer.clone();
    let render_4 = basic_tileset_renderer.clone();

    let menu_state = MenuState::new(vec![
        MenuOption::new("classic".to_string(), Box::new(move || {
            TetrisState::new(Box::new(ClassicDriver::default()), basic_tileset_renderer.clone())
        })),
        MenuOption::new("cascade".to_string(), Box::new(move || {
            TetrisState::new(Box::new(CascadeDriver::default()), render_2.clone())
        })),
        MenuOption::new("sticky".to_string(), Box::new(move || {
            TetrisState::new(Box::new(StickyDriver::default()), render_3.clone())
        })),
        MenuOption::new("fusion".to_string(), Box::new(move || {
            TetrisState::new(Box::new(FusionDriver::default()), render_4.clone())
        })),
        MenuOption::new("options".to_string(), Box::new(|| MenuState::new(vec![]))),
    ]);

    let mut game_states: Vec<Box<dyn GameState>> = vec![menu_state];

    loop {
        // call the current main game state
        let (pop, mut new_states) = game_states.last_mut().unwrap().next_frame();
        for _ in 0..pop {
            game_states.pop();
        }
 
        if !new_states.is_empty() {
            game_states.append(&mut new_states);
        }

        next_frame().await
    }
}
