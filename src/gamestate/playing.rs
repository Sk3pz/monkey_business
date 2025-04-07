use std::time::Duration;

use macroquad::{color::{Color, BLACK, WHITE}, math::vec2, texture::{draw_texture_ex, DrawTextureParams}, window::clear_background};
use macroquad::prelude::{draw_text_ex, measure_text, screen_width};
use macroquad::rand::gen_range;
use macroquad::text::TextParams;
use macroquad::window::screen_height;
use crate::{controls::ControlHandler, player};
use crate::assets::GlobalAssets;
use crate::controls::Action;
use crate::ui::tooltip::{tooltip, ToolTipCard};
use crate::util::{draw_ansi_text, remove_ansii_escape_codes};
use crate::world::example_rock::ExampleRock;
use crate::world::interactable::{Interactable, InteractableAttribute};
use super::{GameState, GameStateAction, GameStateError};

pub struct PlayingGS {
    player: player::Player,
    control_handler: ControlHandler,
    pub interactables: Vec<Box<dyn Interactable>>,
    pub interacting_with: Option<u32>,
    paused: bool,
    debug: bool,
}

impl PlayingGS {
    pub async fn new() -> Result<Box<Self>, GameStateError> {
        let player = player::Player::new().await;
        if let Err(e) = player {
            return Err(GameStateError::InitializationError(format!("Failed to initialize player: {}", e)));
        }
        let player = player.unwrap();

        let control_handler = ControlHandler::load();
        if let Err(e) = control_handler {
            return Err(GameStateError::InitializationError(format!("{}", e)));
        }
        let control_handler = control_handler.unwrap();

        let mut interactables: Vec<Box<dyn Interactable>> = Vec::new();

        // == rock test ==

        for x in 0..5 {
            let rock = ExampleRock::new(x, format!("Rock {}", x), vec2(gen_range(0.0, screen_width()), gen_range(0.0, screen_height())), gen_range(0.0, 360.0));

            interactables.push(Box::new(rock));
        }

        Ok(Box::new(Self {
            player,
            control_handler,
            interactables,
            interacting_with: None,
            paused: false,
            debug: false,
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

    pub fn break_rock(&mut self, id: u32) -> Result<(), GameStateError> {
        // find the rock with the given id
        let mut rock = None;
        for interactable in &mut self.interactables {
            if interactable.get_id() == id {
                rock = Some(interactable);
                break;
            }
        }
        if let Some(rock) = rock {
            // remove the rock from the interactables
            self.interactables.retain(|i| i.get_id() != id);
            // add a new rock to the interactables
            let new_rock = ExampleRock::new(id, format!("Rock {}", id), vec2(gen_range(0.0, screen_width()), gen_range(0.0, screen_height())), gen_range(0.0, 360.0));
            self.interactables.push(Box::new(new_rock));
        }
        Ok(())
    }
}

impl GameState for PlayingGS {

    fn update(&mut self, delta_time: &Duration, assets: &GlobalAssets) -> Result<GameStateAction, GameStateError> {

        // make the player rotate towards the mouse
        // not top down anymore
        //self.player.look_towards_mouse();

        // handle input and make the player respond accordingly
        let actions = self.control_handler.get_actions();
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
                Action::Interact => {
                    let mut clone = self.clone();
                    // check if mouse is on an interactable
                    for interactable in &mut self.interactables {
                        if interactable.is_mouse_over(assets) && interactable.distance_from_player(&self.player, assets) <= 100.0 {
                            self.interacting_with = Some(interactable.get_id());
                            clone.interacting_with = Some(interactable.get_id());
                            let previous_game_state = Some(clone);
                            return interactable.interact(assets, &mut self.player, previous_game_state);
                        }
                    }
                }
                Action::Inventory => {
                    // todo: add an inventory system and open it here
                }
                Action::BasicAttack => {
                    // todo: add attacks
                }
                Action::Debug => {
                    self.debug = !self.debug;
                }
                Action::Pause => {
                    return Ok(GameStateAction::ChangeState(Box::new(super::pause::PauseGS::new(self.clone()))))
                }
                _ => { /* Other actions are not used here */ }
            }
        }
        self.player.apply_movement(movement, &self.interactables, assets, delta_time.as_millis());

        Ok(GameStateAction::NoOp)
    }

    fn pause(&mut self) -> Result<(), GameStateError> {
        self.paused = true;
        Ok(())
    }

    fn restore(&mut self) -> Result<(), GameStateError> {
        // Refresh everything that needs to be
        self.reload_controls()?;
        self.paused = false;
        self.interacting_with = None;
        Ok(())
    }

    fn draw(&self, assets: &GlobalAssets, fps: f32) -> Result<(), GameStateError> {
        // clear the background and give a default color
        clear_background(Color::from_hex(0xf2b888));
        // draw the FPS counter in the top right
        draw_text_ex(&format!("FPS: {}", fps.round()), 2.0, 12.0, TextParams {
            font: Some(&assets.font),
            font_size: 8,
            color: BLACK,
            ..Default::default()
        });

        // draw the player
        draw_texture_ex(
            &self.player.sprite, 
            self.player.pos.x, self.player.pos.y,  
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 32.0)),
                rotation: self.player.rotation,
                ..Default::default()
            }
        );

        // draw the interactables
        for interactable in &self.interactables {
            let interactable_pos = interactable.get_pos();
            draw_texture_ex(
                &interactable.get_sprite(assets),
                interactable_pos.x, interactable_pos.y,
                WHITE,
                DrawTextureParams {
                    rotation: interactable.get_rotation(),
                    ..Default::default()
                }
            );
        }

        if self.debug {
            draw_text_ex(&format!("Player Pos: {}", self.player.pos), 2.0, 28.0, TextParams {
                font: Some(&assets.font),
                font_size: 8,
                color: BLACK,
                ..Default::default()
            });
            draw_text_ex(&format!("Paused: {}", self.paused), 2.0, 44.0, TextParams {
                font: Some(&assets.font),
                font_size: 8,
                color: BLACK,
                ..Default::default()
            });
            let ansi_test = format!("Color Test: {}1{}2{}3{}4{}5{}6{}7{}8{}9{}0{}a{}b{}c{}d{}e{}f",
                                    better_term::Color::BrightBlue,
                                    better_term::Color::Green,
                                    better_term::Color::Cyan,
                                    better_term::Color::Red,
                                    better_term::Color::Purple,
                                    better_term::Color::Yellow,
                                    better_term::Color::White,
                                    better_term::Color::BrightBlack,
                                    better_term::Color::Blue,
                                    better_term::Color::Black,
                                    better_term::Color::BrightGreen,
                                    better_term::Color::BrightCyan,
                                    better_term::Color::BrightRed,
                                    better_term::Color::BrightPurple,
                                    better_term::Color::BrightYellow,
                                    better_term::Color::BrightWhite);
            //debug!("{}", ansi_test.escape_default());
            let raw_ansi_test = remove_ansii_escape_codes(&ansi_test);
            let text_size = measure_text(&raw_ansi_test, Some(&assets.font), 8, 1.0);
            draw_ansi_text(
                &ansi_test,
                vec2(screen_width() - (text_size.width + 10.0), 15.0),
                &assets,
                8,
                4.0,
            );
        }

        if !self.paused {
            // if the mouse is on an interactable, give a tooltip
            for interactable in &self.interactables {
                if interactable.is_mouse_over(assets) {
                    if interactable.distance_from_player(&self.player, assets) <= 100.0 {
                        let interact_btn = self.control_handler.get_binding(&Action::Interact).unwrap();
                        let clicks = match interactable.get_attribute("clicks").unwrap_or(InteractableAttribute::UInt(0)) {
                            InteractableAttribute::UInt(i) => i,
                            _ => 0
                        };
                        let card = ToolTipCard {
                            title: interactable.get_name(),
                            lines: vec![format!("{}Press {}{}{} to interact.", better_term::Color::White,
                                                better_term::Color::BrightYellow, interact_btn, better_term::Color::White),
                                        format!("{}Clicks: {}{}", better_term::Color::White, better_term::Color::BrightYellow, clicks)],
                        };
                        tooltip(card, assets);
                    } else {
                        let card = ToolTipCard {
                            title: interactable.get_name(),
                            lines: vec![format!("{}Get closer to interact!", better_term::Color::White)],
                        };
                        tooltip(card, assets);
                    }
                }
            }
        }

        Ok(())
    }

    fn get_name(&self) -> String {
        "Playing".to_string()
    }

}

impl Clone for PlayingGS {
    fn clone(&self) -> Self {
        let interactables: Vec<Box<dyn Interactable>> =
            self.interactables.iter().map(|i| i.clone()).collect();

        Self {
            player: self.player.clone(),
            control_handler: self.control_handler.clone(),
            interactables,
            interacting_with: self.interacting_with.clone(),
            paused: self.paused,
            debug: self.debug,
        }
    }
}