#![feature(const_fn_floating_point_arithmetic)]

use std::collections::HashMap;
use futures::future::FutureExt;

#[macro_use]
mod game_core;
mod drivers;
mod game_states;
mod ui;

use drivers::*;

use game_states::*;
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
    get_renderer: fn(&mut RenderManagerFactory) -> RenderManagerBuilder
}

impl GameMode {
    fn new(name: &'static str, get_driver: fn() -> Box<dyn Driver>) -> Self {
        Self {
            name, get_driver, 
            get_renderer: |x| x.start_building()
        }
    }

    fn with_get_renderer(mut self, get_renderer: fn(&mut RenderManagerFactory) -> RenderManagerBuilder) -> Self {
        self.get_renderer = get_renderer;
        self
    }

    async fn construct_gamestate<'a>(&self, factory: &mut GameStateManager<'a>) -> Box<dyn GameState<'a> + 'a> {
        TetrisState::new(
            (self.get_driver)(),
            (self.get_renderer)(factory.get_render_manager_factory()).build().await
        ).boxed()
    }
}

#[macroquad::main("TetRust")]
async fn main() {
    let gamemodes = vec![
        GameMode::new("classic", || DriverBuilder::<ClassicDriver>::new().build_boxed())
            .with_get_renderer(|f| f.start_building()
                .with_tilemap("res/basic_tilemap.png", "res/basic_tilemap_info.json")),
        GameMode::new("cascade", || DriverBuilder::<CascadeDriver>::new().build_boxed())
            .with_get_renderer(|f| f.start_building()),
        GameMode::new("sticky", || DriverBuilder::<StickyDriver>::new().build_boxed()),
        GameMode::new("fusion", || DriverBuilder::<FusionDriver>::new().build_boxed())
    ];

    let gamemode_names = gamemodes.iter().map(|gamemode| gamemode.name).collect::<Vec<_>>();
    let gamemodes = gamemodes
        .into_iter()
        .map(|gamemode| (gamemode.name, gamemode))
        .collect::<HashMap::<&'static str, _>>();

    let gamemodes_ref = &gamemodes;
    let mut menu_options = gamemode_names
                .into_iter()
                .map(|name| {
                    MenuOption::new(name.to_string(), move |f| {
                        Box::pin(gamemodes_ref[name].construct_gamestate(f))
                    })
                })
                .collect::<Vec<_>>();
    
    menu_options.push(
        MenuOption::new("options".to_string(), |_| 
            Box::pin(MenuState::new(vec![]).map(|x| x.boxed()))));

    
    let menu_state = MenuState::new(menu_options).await;
    
    let mut gamestate_manager = GameStateManager::new()
        .with_gamestate(menu_state);

    gamestate_manager.run().await;
}
