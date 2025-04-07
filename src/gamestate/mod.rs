use std::{fmt::{self, Display, Formatter}, time::Duration};
use crate::assets::GlobalAssets;

pub mod playing;
pub mod pause;
pub mod example_rock_break_game;

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

    fn update(&mut self, delta_time: &Duration, assets: &GlobalAssets) -> Result<GameStateAction, GameStateError>;
    fn pause(&mut self) -> Result<(), GameStateError>;
    fn restore(&mut self) -> Result<(), GameStateError>;
    fn draw(&self, assets: &GlobalAssets, fps: f32) -> Result<(), GameStateError>;
    fn get_name(&self) -> String;

}