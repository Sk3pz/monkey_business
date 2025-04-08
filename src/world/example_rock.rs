use macroquad::math::Vec2;
use macroquad::prelude::{mouse_position, vec2, Rect};
use crate::assets::GlobalAssets;
use crate::gamedata::GameData;
use crate::gamestate::example_rock_break_game::ExampleRockBreakGameGS;
use crate::gamestate::GameStateAction;
use crate::world::interactable::{Interactable, InteractableAttribute};

#[derive(Clone, Debug)]
pub struct ExampleRock {
    pub id: u32,
    pub name: String,
    pub pos: Vec2,
    pub rotation: f32,
    pub clicks: u32,
}

impl ExampleRock {
    pub fn new(id: u32, name: String, pos: Vec2, rotation: f32) -> Self {
        Self {
            id,
            name,
            pos,
            rotation,
            clicks: 0,
        }
    }
}

impl Interactable for ExampleRock {
    fn interact(&mut self) -> Result<GameStateAction, crate::gamestate::GameStateError> {
        Ok(GameStateAction::PushTopState(
            ExampleRockBreakGameGS::new(self.id)?,
        ))
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_sprite(&self, assets: &GlobalAssets) -> macroquad::prelude::Texture2D {
        assets.rock_sprites.get(0).unwrap().clone()
    }

    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn clone_box(&self) -> Box<dyn Interactable> {
        Box::new(Self {
            id: self.id.clone(),
            name: self.name.clone(),
            pos: self.pos.clone(),
            rotation: self.rotation.clone(),
            clicks: self.clicks.clone(),
        })
    }

    fn is_mouse_over(&self, data: &GameData) -> bool {
        let mouse_pos = vec2(mouse_position().0, mouse_position().1);
        let rect = Rect::new(self.pos.x, self.pos.y, self.get_sprite(&data.assets).width(), self.get_sprite(&data.assets).height());
        rect.contains(mouse_pos)
    }

    fn distance_from_player(&self, data: &GameData) -> f32 {
        let player = &data.world.player;
        let player_pos = vec2(player.pos.x + player.sprite.width() / 2.0, player.pos.y + player.sprite.height() / 2.0);
        let sprite = self.get_sprite(&data.assets);
        let interactable_pos = vec2(self.pos.x + sprite.width() / 2.0, self.pos.y + sprite.height() / 2.0);
        let dx = interactable_pos.x - player_pos.x;
        let dy = interactable_pos.y - player_pos.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn get_attribute(&self, attribute: &str) -> Option<InteractableAttribute> {
        match attribute {
            "clicks" => Some(InteractableAttribute::UInt(self.clicks)),
            _ => None,
        }
    }

    fn set_attribute(&mut self, attribute: &str, value: InteractableAttribute) -> Result<(), String> {
        match attribute {
            "clicks" => {
                if let InteractableAttribute::UInt(v) = value {
                    self.clicks = v;
                    Ok(())
                } else {
                    Err(format!("Invalid type for attribute {}: expected UInt", attribute))
                }
            }
            _ => Err(format!("Unknown attribute: {}", attribute)),
        }
    }
}