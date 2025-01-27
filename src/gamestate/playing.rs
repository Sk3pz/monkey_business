use std::time::Duration;

use macroquad::{color::{Color, BLACK, WHITE}, math::vec2, text::draw_text, texture::{draw_texture_ex, DrawTextureParams}, window::clear_background};

use crate::{controls::{Action, ControlHandler}, player};

use super::{GameState, GameStateAction, GameStateError};

#[derive(Clone)]
pub struct PlayingGS {
    player: player::Player,
    control_handler: ControlHandler,
}

impl PlayingGS {
    pub async fn new() -> Result<Box<Self>, GameStateError> {
        let player = player::Player::new().await;
        if let Err(e) = player {
            return Err(GameStateError::InitializationError(format!("Failed to initialize player: {}", e)));
        }
        let player = player.unwrap();

        let control_handler = ControlHandler::load();

        Ok(Box::new(Self {
            player,
            control_handler,
        }))
    }

    pub fn reload_controls(&mut self) {
        self.control_handler = ControlHandler::load();
    }
}

impl GameState for PlayingGS {

    fn update(&mut self, delta_time: &Duration) -> Result<GameStateAction, GameStateError> {

        // make the player rotate towards the mouse
        self.player.look_towards_mouse();

        // handle input and make the player respond accordingly
        let actions = self.control_handler.get_actions_down();
        let mut movement = vec2(0.0, 0.0);
        // handle various movement types
        for action in actions {
            match action {
                // todo: add limits like obstacles
                Action::MoveUp => {
                    movement.y -= 1.0;
                }
                Action::MoveDown => {
                    movement.y += 1.0;
                }
                Action::MoveLeft => {
                    movement.x -= 1.0;
                }
                Action::MoveRight => {
                    movement.x += 1.0;
                }
                _ => {}
            }
        }
        self.player.apply_movement(movement, delta_time.as_millis());

        // handle the pause key with a key release to prevent spamming
        let actions = self.control_handler.get_actions_up();
        for action in actions {
            match action {
                Action::Pause => {
                    return Ok(GameStateAction::ChangeState(Box::new(super::pause::PauseGS::new(self.clone()))))
                }
                _ => {}
            }
        }

        Ok(GameStateAction::NoOp)
    }

    fn draw(&self, fps: f32) -> Result<(), GameStateError> {
        // clear the background and give a default color
        clear_background(Color::from_rgba(222, 192, 138, 255));
        // draw the FPS counter in the top right
        draw_text(&format!("FPS: {}", fps.round()), 2.0, 12.0, 20.0, BLACK);

        // draw the player
        draw_texture_ex(
            &self.player.sprite, 
            self.player.pos.x, self.player.pos.y,  
            WHITE,
            DrawTextureParams {
                rotation: self.player.rotation,
                ..Default::default()
            }
        );

        Ok(())
    }

}