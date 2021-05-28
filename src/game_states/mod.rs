use async_trait::async_trait;

pub mod menu_state;
pub mod tetris_state;

use crate::ui::rendering::RenderManagerFactory;


/**
 * This is the main driver of the entire game.
 */
pub struct GameStateManager<'a> {
    gamestate_stack: Vec<Box<dyn GameState<'a> + 'a>>,
    render_manager_factory: RenderManagerFactory
}

impl<'a> GameStateManager<'a> {
    pub fn new() -> Self {
        Self {
            gamestate_stack: Vec::new(),
            render_manager_factory: RenderManagerFactory::new(),
        }
    }

    pub fn with_gamestate(mut self, gamestate: impl GameState<'a> + 'a) -> Self {
        self.gamestate_stack.push(Box::new(gamestate));
        self
    }

    pub async fn run(&mut self) {
        while let Some(gamestate) = self.gamestate_stack.pop() {
            gamestate.run(self).await;
        }
    }

    pub fn get_gamestate_stack(&mut self) -> &mut Vec<Box<dyn GameState<'a> + 'a>> {
        &mut self.gamestate_stack
    }

    pub fn get_render_manager_factory(&mut self) -> &mut RenderManagerFactory {
        &mut self.render_manager_factory
    }
}

#[async_trait(?Send)]
pub trait GameState<'a> {
    // take control of execution, and fully manage the gamestate
    async fn run(
        self: Box<Self>, 
        gamestate_manager: &mut GameStateManager<'a>
    );

    fn boxed(self) -> Box<dyn GameState<'a> + 'a> where Self: 'a + Sized {
        Box::new(self)
    }
}