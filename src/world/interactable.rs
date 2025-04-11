use macroquad::prelude::*;
use crate::animation::Animator;
use crate::error::GameError;
use crate::gamedata::GameData;
use crate::gamestate::GameStateAction;

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
    fn is_mouse_over(&self, data: &GameData) -> bool;
    fn distance_from_player(&self, data: &GameData) -> f32;
    fn get_attribute(&self, attribute: &str) -> Option<InteractableAttribute>;
    fn set_attribute(&mut self, attribute: &str, value: InteractableAttribute) -> Result<(), String>;
}

impl Clone for Box<dyn Interactable> {
    fn clone(&self) -> Box<dyn Interactable> {
        self.clone_box()
    }
}