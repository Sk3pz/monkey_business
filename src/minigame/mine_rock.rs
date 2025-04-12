use std::time::Duration;
use macroquad::color::Color;
use macroquad::input::mouse_position;
use macroquad::math::{vec2, Rect};
use macroquad::prelude::{draw_rectangle, screen_height, screen_width};
use macroquad::rand::gen_range;
use crate::controls::Action;
use crate::error::GameError;
use crate::gamedata::GameData;
use crate::overlay::{Overlay, OverlayAction};
use crate::world::rock::Rock;
use crate::world::interactable::InteractableAttribute;

#[derive(Debug)]
pub struct MineRock {
    rock_id: u32,
    clicks: u32,
    recently_clicked: bool,
}

impl MineRock {
    pub fn new(rock_id: u32) -> Result<Box<Self>, GameError> {
        Ok(Box::new(Self {
            rock_id,
            clicks: 0,
            recently_clicked: false,
        }))
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

impl Overlay for MineRock {

    fn init(&mut self, data: &mut GameData) -> Result<(), GameError> {
        let Some(rock) = data.world.get_interactable_by_id(self.rock_id) else {
            return Err(GameError::Initialization(format!("Rock with id {} not found", self.rock_id)));
        };

        self.clicks = if let Some(InteractableAttribute::UInt(clicks)) = rock.get_attribute("clicks") {
            clicks
        } else {
            0
        };

        Ok(())
    }

    fn update(&mut self, _delta_time: &Duration, data: &mut GameData) -> Result<OverlayAction, GameError> {
        self.recently_clicked = false;
        // handle on release to ensure pause key isn't spammed when held (was an issue)
        let actions = data.control_handler.get_actions();
        for action in actions {
            match action {
                Action::Pause => {
                    return Ok(OverlayAction::Exit);
                }
                Action::BasicAttack => {
                    if self.is_click_inside_rock() {
                        self.recently_clicked = true;
                    }
                }
                Action::UIClick => {
                    if self.is_click_inside_rock() {
                        // increment clicks
                        self.clicks += 1;
                        // write the clicks to the rock
                        let Some(rock) = data.world.get_mut_interactable_by_id(self.rock_id) else {
                            return Err(GameError::Update(format!("Rock with id {} not found", self.rock_id)));
                        };
                        if let Err(e) = rock.set_attribute("clicks", InteractableAttribute::UInt(self.clicks)) {
                            return Err(GameError::Update(format!("Failed to set clicks: {}", e)));
                        }
                        if self.clicks >= 16 {
                            if let Err(e) = data.world.break_interactable(self.rock_id) {
                                return Err(GameError::Update(format!("Failed to break rock: {}", e)));
                            }

                            // add a new rock to the world
                            let new_rock = Rock::new(&data.assets, self.rock_id,
                                                     "Rock Pile".to_string(),
                                                     vec2(gen_range(20.0, screen_width() - 20.0), gen_range(20.0, screen_height() - 20.0)),
                                                     gen_range(0.0, 360.0));

                            data.world.add_interactable(Box::new(new_rock));

                            return Ok(OverlayAction::Exit);
                        }
                    }
                    self.recently_clicked = false;
                }
                _ => {}
            }
        }
        Ok(OverlayAction::NoOp)
    }

    fn draw(&self, data: &mut GameData) -> Result<(),GameError> {
        let rock_sprite = {
            // get the rock
            let rock = data.world.get_interactable_by_id(self.rock_id);
            if let Some(rock) = rock {
                rock.get_animator()
            } else {
                return Err(GameError::Draw(format!("Rock with id {} not found", self.rock_id)));
            }
        };

        // draw a semi-transparent overlay
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(61,51,51,150));

        let scale = if self.recently_clicked {
            vec2(300.0, 300.0)
        } else {
            vec2(256.0, 256.0)
        };

        // get the rock
        let Some(rock) = data.world.get_interactable_by_id(self.rock_id) else {
            return Err(GameError::Draw(format!("Rock with id {} not found", self.rock_id)));
        };

        // draw the rock at the center of the screen
        rock_sprite.draw(vec2(screen_width() / 2.0 - scale.x / 2.0,
                              screen_height() / 2.0 - scale.y / 2.0),
        Some(rock.get_rotation()), Some(scale));

        // draw large text at the bottom of the screen displaying the number of clicks
        // let text = format!("Clicks: {}", self.clicks);
        // let text_size = measure_text(&text, Some(&data.assets.font), 32, 1.0);
        // draw_text_ex(&text,
        //              screen_width() / 2.0 - text_size.width / 2.0,
        //              screen_height() - text_size.height * 2.0,
        //              macroquad::text::TextParams {
        //                  font: Some(&data.assets.font),
        //                  font_size: 32,
        //                  color: WHITE,
        //                  ..Default::default()
        //              });

        Ok(())
    }

    fn draw_below(&self) -> bool {
        true
    }
}