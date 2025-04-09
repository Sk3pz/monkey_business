use std::{fmt::{self, Display, Formatter}, time::Duration};
use std::fmt::Debug;
use crate::gamedata::GameData;
use crate::overlay::Overlay;

pub mod playing;
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
    SpawnOverlay(Box<dyn Overlay>),
    NoOp,
}

pub trait GameState: Debug {

    /// updates when the game is active
    /// Great for key presses, mouse movement, etc.
    fn update(&mut self, delta_time: &Duration, data: &mut GameData) -> Result<GameStateAction, GameStateError>;
    /// Will always be called, even when an overlay is shown
    fn persistent_update(&mut self, delta_time: &Duration, data: &mut GameData) -> Result<GameStateAction, GameStateError>;
    fn pause(&mut self, data: &mut GameData) -> Result<(), GameStateError>;
    fn restore(&mut self, data: &mut GameData) -> Result<(), GameStateError>;
    fn draw(&self, data: &mut GameData) -> Result<(), GameStateError>;
    fn get_name(&self) -> String;
    fn is_overlay(&self) -> bool {
        false
    }
}