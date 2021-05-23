#![feature(const_fn_floating_point_arithmetic)]

use macroquad::prelude::*;
use std::pin::Pin;
use std::collections::HashMap;
use std::rc::Rc;
use futures::future::{FutureExt, ready};

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
use debugging::recording::RecordingDriver;
use debugging::replaying::ReplayingDriver;

use ui::rendering::*;


struct GameMode {
    name: &'static str,
    get_driver: fn() -> Box<dyn Driver>,
    get_renderer: for <'a> fn(&'a mut RenderManagerFactory) -> Pin<Box<dyn FutureExt<Output = RenderManager<'a>> + 'a>>
}

impl GameMode {
    fn new(name: &'static str, get_driver: fn() -> Box<dyn Driver>) -> Self {
        Self {
            name, get_driver, 
            get_renderer: |x| Box::pin(x.start_building().build())
        }
    }

    fn with_get_renderer(mut self, get_renderer: for <'a> fn(&'a mut RenderManagerFactory) -> Pin<Box<dyn FutureExt<Output = RenderManager<'a>> + 'a>>) -> Self {
        self.get_renderer = get_renderer;
        self
    }

    async fn construct_gamestate<'a>(&self, factory: &'a mut RenderManagerFactory) -> Box<dyn GameState + 'a> {
        TetrisState::new(
            (self.get_driver)(),
            (self.get_renderer)(factory).await
        )
    }
}

#[macroquad::main("TetRust")]
async fn main() {
    let mut render_manager_factory = Rc::new(RenderManagerFactory::new());

    let gamemodes = vec![
        GameMode::new("classic", || DriverBuilder::<ClassicDriver>::new().build_boxed())
            .with_get_renderer(|f| Box::pin(f.start_building()
                .with_tilemap("res/basic_tilemap.png", "res/basic_tilemap_info.json")
                .build())),
        GameMode::new("cascade", || DriverBuilder::<CascadeDriver>::new().build_boxed())
            .with_get_renderer(|f| Box::pin(f.start_building().build())),
        GameMode::new("sticky", || DriverBuilder::<StickyDriver>::new().build_boxed()),
        GameMode::new("fusion", || DriverBuilder::<FusionDriver>::new().build_boxed())
    ]
    .into_iter()
    .map(|gamemode| (gamemode.name, gamemode))
    .collect::<HashMap::<_, _>>();

    let mut menu_options = gamemodes
                .keys()
                .into_iter()
                .map(|name| {
                    let factory_ref = render_manager_factory.clone();
                    MenuOption::new(name.to_string(), Box::new(move || {
                        let factory = Rc::get_mut(&mut factory_ref).unwrap();
                        gamemodes[name].construct_gamestate(factory)
                    }))
                })
                .collect::<Vec<_>>();

    let menu_state = MenuState::new(vec![
        MenuOption::new("options".to_string(), Box::new(|| MenuState::new(vec![])))
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
