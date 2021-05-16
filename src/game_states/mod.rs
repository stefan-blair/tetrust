pub mod menu_state;
pub mod tetris_state;


pub trait GameState {
    // returns a tuple, the number of states to pop and any states to push
    fn next_frame(&mut self) -> (usize, Vec<Box<dyn GameState>>);
}