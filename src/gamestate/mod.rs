use std::fmt::Debug;
use std::time::Duration;
use crate::error::GameError;
use crate::gamedata::GameData;
use crate::overlay::Overlay;

pub mod playing;

pub enum GameStateAction {
    ChangeState(Box<dyn GameState>),
    SpawnOverlay(Box<dyn Overlay>),
    NoOp,
}

pub trait GameState: Debug {

    /// updates when the game is active
    /// Great for key presses, mouse movement, etc.
    fn update(&mut self, delta_time: &Duration, data: &mut GameData) -> Result<GameStateAction, GameError>;
    /// Will always be called, even when an overlay is shown
    fn persistent_update(&mut self, delta_time: &Duration, data: &mut GameData) -> Result<GameStateAction, GameError>;
    fn pause(&mut self, data: &mut GameData) -> Result<(), GameError>;
    fn restore(&mut self, data: &mut GameData) -> Result<(), GameError>;
    fn draw(&mut self, delta_time: &Duration, data: &mut GameData) -> Result<(), GameError>;
    fn get_name(&self) -> String;
    fn is_overlay(&self) -> bool {
        false
    }
}