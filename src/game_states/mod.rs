pub mod menu_state;
pub mod tetris_state;


pub trait GameState {
    fn next_frame(&mut self) -> (bool, Vec<Box<dyn GameState>>);
}