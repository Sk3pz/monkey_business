use std::{fmt::{self, Display, Formatter}, time::Duration};
use std::fmt::Debug;
use crate::assets::GlobalAssets;
use crate::gamedata::GameData;

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
    PushTopState(Box<dyn GameState>),
    PopTopState,
    NoOp,
}

pub trait GameState: Debug {

    fn update(&mut self, delta_time: &Duration, data: &mut GameData) -> Result<GameStateAction, GameStateError>;
    fn pause(&mut self, data: &mut GameData) -> Result<(), GameStateError>;
    fn restore(&mut self, data: &mut GameData) -> Result<(), GameStateError>;
    fn draw(&self, data: &mut GameData) -> Result<(), GameStateError>;
    fn get_name(&self) -> String;
    fn is_overlay(&self) -> bool {
        false
    }
}

pub struct GameStateManager {
    pub play_state: Box<dyn GameState>,
    pub top_states: Vec<Box<dyn GameState>>,
}

impl GameStateManager {
    pub fn new(play_state: Box<dyn GameState>) -> Self {
        Self {
            play_state,
            top_states: Vec::new(),
        }
    }

    pub fn get_immutable_top_state(&self) -> &Box<dyn GameState> {
        if let Some(s) = self.top_states.last() {
            s
        } else {
            &self.play_state
        }
    }

    pub fn get_top_state(&mut self) -> &mut Box<dyn GameState> {
        if let Some(s) = self.top_states.last_mut() {
            s
        } else {
            &mut self.play_state
        }
    }

    pub fn change_play_state(&mut self, new_state: Box<dyn GameState>, data: &mut GameData) -> Result<(), GameStateError> {
        // pause the current state
        if let Err(e) = self.get_top_state().pause(data) {
            return Err(GameStateError::RuntimeError(format!("Failed to pause gamestate: {}", e)));
        }
        // change the play state
        self.play_state = new_state;
        self.play_state.restore(data)
    }

    pub fn push_top_state(&mut self, state: Box<dyn GameState>, data: &mut GameData) -> Result<(), GameStateError> {
        // pause the current state
        self.get_top_state().pause(data)?;
        self.top_states.push(state);
        // restore the new state
        self.get_top_state().restore(data)
    }

    pub fn pop_top_state(&mut self, data: &mut GameData) -> Result<(), GameStateError> {
        // pause the current state
        self.get_top_state().pause(data)?;
        self.top_states.pop();
        // restore the new state
        self.get_top_state().restore(data)
    }
}