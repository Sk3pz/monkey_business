use std::time::Duration;

use macroquad::{color::Color, shapes::draw_rectangle, window::{screen_height, screen_width}};
use crate::assets::GlobalAssets;
use crate::controls::{Action, ControlHandler};
use crate::debug;
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

    fn update(&mut self, _delta_time: &Duration, assets: &GlobalAssets) -> Result<GameStateAction, GameStateError> {
        let control_handler = ControlHandler::load();
        if let Err(e) = control_handler {
            return Err(GameStateError::RuntimeError(e));
        }
        let control_handler = control_handler.unwrap();
        // handle on release to ensure pause key isn't spammed when held (was an issue)
        let actions = control_handler.get_actions();
        for action in actions {
            match action {
                Action::Pause => {
                    if let Err(e) = self.previous_play_state.restore() {
                        return Err(e);
                    }
                    return Ok(GameStateAction::ChangeState(Box::new(self.previous_play_state.clone())));
                }
                _ => {
                    // do nothing
                    debug!("PauseGS: ignoring action {:?}", action);
                }
            }
        }
        Ok(GameStateAction::NoOp)
    }

    fn restore(&mut self) -> Result<(), GameStateError> {
        Ok(())
    }

    fn draw(&self, assets: &GlobalAssets, fps: f32) -> Result<(), GameStateError> {
        // draw the player in the correct position
        // todo: this will not show updates to other players surrounding the player when networking is implemented, the update function may need to be called with a special pause flag?
        self.previous_play_state.draw(assets, fps)?;

        // draw a semi-transparent overlay
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.5));

        Ok(())
    }
    
}