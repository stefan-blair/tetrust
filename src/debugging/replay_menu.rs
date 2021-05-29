use std::collections::HashMap;

use crate::game_states::tetris_state::TetrisState;
use crate::debugging::drivers::replaying::ReplayingDriver;
use crate::game_states::menu_state::*;
use crate::game_states::*;
use crate::GameMode;
use super::*;


async fn construct_gamestate_replay<'a>(gamemode: &GameMode, factory: &mut GameStateManager<'a>, replay: String) -> Box<dyn GameState<'a> + 'a> {
    TetrisState::new(
        Box::new(ReplayingDriver::new((gamemode.get_driver)(), &replay)),
        (gamemode.get_renderer)(factory.get_render_manager_factory()).build().await
    ).boxed()
}

pub async fn get_replay_menu<'a>(gamemodes: &'a HashMap<&str, GameMode>) -> Box<dyn GameState<'a> + 'a> {
    let recordings = recording_manager::get_sorted_recordings();
    let menu_options = recordings
        .into_iter()
        .map(|entry| MenuOption::new(
            format!("{}_{}", entry.gamemode_name, entry.gamemode_index),
            move |f|
                Box::pin(construct_gamestate_replay(&gamemodes[entry.gamemode_name.as_str()], f, entry.to_filename()))
        ))
        .collect::<Vec<_>>();

    Box::new(MenuState::new(menu_options).await)
}