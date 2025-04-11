use macroquad::prelude::*;
use crate::animation::Animator;
use crate::error::GameError;
use crate::gamedata::GameData;
use crate::gamestate::GameStateAction;
use crate::util::{get_sprite_scale, scale_position};

pub enum InteractableAttribute {
    Int(i32),
    UInt(u32),
    Float(f32),
    String(String),
    Bool(bool),
}

pub trait Interactable {
    fn interact(&mut self) -> Result<GameStateAction, GameError>;
    fn get_name(&self) -> String;
    fn get_sprite_size(&self) -> Vec2;
    fn get_animator(&self) -> &Animator;
    fn update_animation(&mut self, delta_time: f32) -> Result<(), GameError>;
    fn draw(&self, data: &GameData) -> Result<(), GameError>;
    fn get_pos(&self) -> Vec2;
    fn get_id(&self) -> u32;
    fn get_rotation(&self) -> f32;
    fn clone_box(&self) -> Box<dyn Interactable>;
    fn distance_from_player(&self, data: &GameData) -> f32;
    fn get_attribute(&self, attribute: &str) -> Option<InteractableAttribute>;
    fn set_attribute(&mut self, attribute: &str, value: InteractableAttribute) -> Result<(), String>;

    fn get_scaled_pos(&self) -> Vec2 {
        scale_position(self.get_pos())
    }

    fn is_mouse_over(&self, _data: &GameData) -> bool {
        let mouse_pos = vec2(mouse_position().0, mouse_position().1);
        let sprite_scale = get_sprite_scale();
        let sprite_size = self.get_sprite_size();
        let pos = self.get_scaled_pos();
        let rect = Rect::new(pos.x, pos.y, sprite_scale.x, sprite_scale.y);
        rect.contains(mouse_pos)
    }
}

impl Clone for Box<dyn Interactable> {
    fn clone(&self) -> Box<dyn Interactable> {
        self.clone_box()
    }
}