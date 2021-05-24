use futures::future::FutureExt;

pub mod menu_state;
pub mod tetris_state;


pub trait GameState {
    // take control of execution, and fully manage the gamestate
    fn run(self, gamestates: &mut Vec<Box<dyn GameState>>) -> Box<dyn FutureExt<Output = ()>>;
}