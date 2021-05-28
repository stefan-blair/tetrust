use std::collections::HashMap;

use crate::game_states::menu_state::*;
use crate::game_states::*;
use crate::GameMode;
use super::*;


pub async fn get_replay_menu<'a>(gamemodes: &'a HashMap<&str, GameMode>) -> Box<dyn GameState<'a> + 'a> {
    let recordings = recording_manager::get_sorted_recordings();
    let menu_options = recordings
        .into_iter()
        .map(|entry| MenuOption::new(
            format!("{}_{}", entry.gamemode_name, entry.gamemode_index),
            move |f|
                Box::pin(
                    gamemodes[entry.gamemode_name.as_str()]
                        .construct_gamestate_replay(f, entry.to_filename()))
        ))
        .collect::<Vec<_>>();

    Box::new(MenuState::new(menu_options).await)
}