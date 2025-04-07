use std::time::Duration;

use macroquad::{color::Color, shapes::draw_rectangle, window::{screen_height, screen_width}};
use macroquad::color::WHITE;
use macroquad::input::mouse_position;
use macroquad::math::{vec2, Rect, Vec2};
use macroquad::prelude::{draw_texture_ex};
use macroquad::text::{draw_text_ex, measure_text};
use macroquad::texture::DrawTextureParams;
use crate::assets::GlobalAssets;
use crate::controls::{Action, ControlHandler};
use crate::gamestate::playing::PlayingGS;
use crate::world::interactable::InteractableAttribute;
use super::{GameState, GameStateAction, GameStateError};

// todo: this is just an example of the engine

pub struct ExampleRockBreakGameGS {
    previous_play_state: PlayingGS,
    clicks: u32,
    control_handler: ControlHandler,
    recently_clicked: bool,
    rock_id: u32,
}

impl ExampleRockBreakGameGS {
    pub fn new(previous_play_state: PlayingGS) -> Result<Box<Self>, GameStateError> {
        let control_handler = ControlHandler::load();
        if let Err(e) = control_handler {
            return Err(GameStateError::InitializationError(format!("{}", e)));
        }
        let control_handler = control_handler.unwrap();

        // get clicks
        let clicks = previous_play_state.interacting_with
            .and_then(|id| previous_play_state.interactables.iter()
                .find(|i| i.get_id() == id)
                .and_then(|i| i.get_attribute("clicks"))
                .and_then(|a| match a {
                    InteractableAttribute::UInt(c) => Some(c),
                    _ => None,
                }))
            .unwrap_or(0);

        Ok(Box::new(Self {
            rock_id: previous_play_state.interacting_with.unwrap_or(0),
            previous_play_state,
            clicks,
            control_handler,
            recently_clicked: false,
        }))
    }

    pub fn reload_controls(&mut self) -> Result<(), GameStateError> {
        let new_ct_handler = ControlHandler::load();
        if let Err(e) = new_ct_handler {
            return Err(GameStateError::InitializationError(format!("{}", e)));
        }
        let new_ct_handler = new_ct_handler.unwrap();
        self.control_handler = new_ct_handler;
        Ok(())
    }

    fn is_click_inside_rock(&self) -> bool {
        let mouse_pos = vec2(mouse_position().0, mouse_position().1);
        let rock_scale = if self.recently_clicked {
            vec2(300.0, 300.0)
        } else {
            vec2(256.0, 256.0)
        };
        let rock_pos = vec2(screen_width() / 2.0 - rock_scale.x / 2.0,
                            screen_height() / 2.0 - rock_scale.y / 2.0);
        let rock_rect = Rect::new(rock_pos.x, rock_pos.y, rock_scale.x, rock_scale.y);
        rock_rect.contains(mouse_pos)
    }
}

impl GameState for ExampleRockBreakGameGS {

    fn update(&mut self, _delta_time: &Duration, assets: &GlobalAssets) -> Result<GameStateAction, GameStateError> {
        self.recently_clicked = false;
        let rock_sprite = assets.rock_sprites.get(0).unwrap();
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
                    if let Some(rock) = self.previous_play_state.interacting_with {
                        // update the interactable with id rock_id with the clicks
                        self.previous_play_state.interactables
                            .iter_mut()
                            .filter(|i| i.get_id() == rock)
                            .for_each(|i| {
                                i.set_attribute("clicks", InteractableAttribute::UInt(self.clicks)).unwrap();
                            });
                    }
                    if let Err(e) = self.previous_play_state.restore() {
                        return Err(e);
                    }
                    return Ok(GameStateAction::ChangeState(Box::new(self.previous_play_state.clone())));
                }
                Action::BasicAttack => {
                    if self.is_click_inside_rock() {
                        self.recently_clicked = true;
                    }
                }
                Action::UIClick => {
                    // check if click on the rock, which is a 256x256 sprite in the center of the screen
                    let rock_pos = vec2(screen_width() / 2.0 - rock_sprite.width() / 2.0,
                                    screen_height() / 2.0 - rock_sprite.height() / 2.0);
                    if self.is_click_inside_rock() {
                        // increment clicks
                        self.clicks += 1;
                        if self.clicks >= 10 {
                            self.previous_play_state.break_rock(self.rock_id)?;
                            return Ok(GameStateAction::ChangeState(Box::new(self.previous_play_state.clone())));
                        }
                    }
                    self.recently_clicked = false;
                }
                _ => {}
            }
        }
        Ok(GameStateAction::NoOp)
    }

    fn pause(&mut self) -> Result<(), GameStateError> {
        Ok(())
    }

    fn restore(&mut self) -> Result<(), GameStateError> {
        self.previous_play_state.pause()?;
        Ok(())
    }

    fn draw(&self, assets: &GlobalAssets, fps: f32) -> Result<(), GameStateError> {
        // draw the player in the correct position
        self.previous_play_state.draw(assets, fps)?;

        let rock_sprite = assets.rock_sprites.get(0).unwrap();

        // draw a semi-transparent overlay
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(255, 255, 255, 80));

        let scale = if self.recently_clicked {
            vec2(300.0, 300.0)
        } else {
            vec2(256.0, 256.0)
        };

        // get the rock
        let rock = self.previous_play_state.interactables.iter().find(|i| i.get_id() == self.rock_id).unwrap();

        // draw the rock at the center of the screen
        draw_texture_ex(
            &rock_sprite,
            screen_width() / 2.0 - scale.x / 2.0,
            screen_height() / 2.0 - scale.y / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(scale),
                source: None,
                rotation: rock.get_rotation(),
                ..Default::default()
            }
        );

        // draw large text at the bottom of the screen displaying the number of clicks
        let text = format!("Clicks: {}", self.clicks);
        let text_size = measure_text(&text, Some(&assets.font), 32, 1.0);
        draw_text_ex(&text,
                      screen_width() / 2.0 - text_size.width / 2.0,
                      screen_height() - text_size.height * 2.0,
                      macroquad::text::TextParams {
                          font: Some(&assets.font),
                          font_size: 32,
                          color: WHITE,
                          ..Default::default()
                      });

        Ok(())
    }

    fn get_name(&self) -> String {
        "ExampleRockBreakGame".to_string()
    }

}