use std::fmt::Display;
use macroquad::prelude::*;
use crate::assets::GlobalAssets;
use crate::gamestate::{GameStateAction, GameStateError};
use crate::gamestate::playing::PlayingGS;
use crate::player::Player;

pub enum InteractableAttribute {
    Int(i32),
    UInt(u32),
    Float(f32),
    String(String),
    Bool(bool),
}

pub trait Interactable {
    fn interact(&mut self, assets: &GlobalAssets, player: &mut Player, previous_game_state: Option<PlayingGS>) -> Result<GameStateAction, GameStateError>;
    fn get_name(&self) -> String;
    fn get_sprite(&self, assets: &GlobalAssets) -> Texture2D;
    fn get_pos(&self) -> Vec2;
    fn get_id(&self) -> u32;
    fn get_rotation(&self) -> f32;
    fn clone_box(&self) -> Box<dyn Interactable>;
    fn is_mouse_over(&self, assets: &GlobalAssets) -> bool;
    fn distance_from_player(&self, player: &Player, assets: &GlobalAssets) -> f32;
    fn get_attribute(&self, attribute: &str) -> Option<InteractableAttribute>;
    fn set_attribute(&mut self, attribute: &str, value: InteractableAttribute) -> Result<(), String>;
}

impl Clone for Box<dyn Interactable> {
    fn clone(&self) -> Box<dyn Interactable> {
        self.clone_box()
    }
}

// #[derive(Clone, Debug)]
// pub struct Interactable {
//     pub name: String,
//     pub pos: Vec2,
//     pub sprite: Texture2D,
//     pub rotation: f32,
//
//     interaction: fn(assets: &GlobalAssets, &mut Player, previous_game_state: Option<PlayingGS>) -> Result<GameStateAction, GameStateError>,
// }
//
// impl Interactable {
//     pub fn new(
//         name: String,
//         pos: Vec2,
//         sprite: Texture2D,
//         rotation: f32,
//         interaction: fn(assets: &GlobalAssets, &mut Player, previous_game_state: Option<PlayingGS>) -> Result<GameStateAction, GameStateError>,
//     ) -> Self {
//         Self {
//             name,
//             pos,
//             sprite,
//             rotation,
//             interaction,
//         }
//     }
//
//     pub fn interact(&mut self, assets: &GlobalAssets, player: &mut Player, previous_game_state: Option<PlayingGS>) -> Result<GameStateAction, GameStateError> {
//         (self.interaction)(assets, player, previous_game_state)
//     }
//
//     pub fn distance_from_player(&self, player: &Player) -> f32 {
//         let player_pos = vec2(player.pos.x + player.sprite.width() / 2.0, player.pos.y + player.sprite.height() / 2.0);
//         let interactable_pos = vec2(self.pos.x + self.sprite.width() / 2.0, self.pos.y + self.sprite.height() / 2.0);
//         let dx = interactable_pos.x - player_pos.x;
//         let dy = interactable_pos.y - player_pos.y;
//         (dx * dx + dy * dy).sqrt()
//     }
//
//     pub fn is_mouse_over(&self) -> bool {
//         let mouse_pos = vec2(mouse_position().0, mouse_position().1);
//         let rect = Rect::new(self.pos.x, self.pos.y, self.sprite.width(), self.sprite.height());
//         rect.contains(mouse_pos)
//         // let pos = vec2(self.pos.x + self.sprite.width() / 2.0, self.pos.y + self.sprite.height() / 2.0);
//         //
//         // let dx = mouse_pos.0 - pos.x;
//         // let dy = mouse_pos.1 - pos.y;
//         //
//         // dx * dx + dy * dy < (self.sprite.width() / 2.0).powi(2)
//     }
// }
//
// impl Display for Interactable {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Interactable '{}'({})", self.name, self.pos)
//     }
// }