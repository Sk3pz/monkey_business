use std::time::Duration;

use macroquad::{color::BLUE, window::clear_background};

use crate::controls::ControlHandler;

use super::{playing::PlayingGS, GameState, GameStateAction, GameStateError};

pub struct PauseGS {
    previous_play_state: PlayingGS,
}

impl PauseGS {
    pub fn new(previous_play_state: PlayingGS) -> Self {
        Self {
            previous_play_state,
        }
    }
}

impl GameState for PauseGS {

    fn update(&mut self, _delta_time: &Duration) -> Result<GameStateAction, GameStateError> {
        let control_handler = ControlHandler::load();
        // handle on release to ensure pause key isnt spammed when held (was an issue)
        let actions = control_handler.get_keys_up();
        for action in actions {
            match action {
                crate::controls::Action::Pause => {
                    self.previous_play_state.reload_controls();
                    return Ok(GameStateAction::ChangeState(Box::new(self.previous_play_state.clone())));
                }
                _ => {}
            }
        }
        Ok(GameStateAction::NoOp)
    }

    fn draw(&self, _fps: f32) -> Result<(), GameStateError> {
        clear_background(BLUE);
        Ok(())
    }
    
}