use std::{fmt::{self, Display, Formatter}, time::Duration};

pub mod playing;
pub mod pause;

#[derive(Debug, PartialEq, Eq)]
pub enum GameStateError {
    InitializationError(String),
    RuntimeError(String),
}

impl Display for GameStateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameStateError::InitializationError(e) => write!(f, "InitializationError: {}", e),
            GameStateError::RuntimeError(e) => write!(f, "RuntimeError: {}", e),
        }
    }
}

pub enum GameStateAction {
    ChangeState(Box<dyn GameState>),
    NoOp,
}

pub trait GameState {

    fn update(&mut self, delta_time: &Duration) -> Result<GameStateAction, GameStateError>;
    fn draw(&self, fps: f32) -> Result<(), GameStateError>;

}