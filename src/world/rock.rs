use macroquad::math::Vec2;
use macroquad::prelude::{mouse_position, vec2, Rect};
use crate::animation::Animator;
use crate::assets::GlobalAssets;
use crate::error::GameError;
use crate::gamedata::GameData;
use crate::gamestate::GameStateAction;
use crate::minigame::mine_rock::MineRock;
use crate::world::interactable::{Interactable, InteractableAttribute};

#[derive(Clone, Debug)]
pub struct Rock {
    pub id: u32,
    pub name: String,
    pub pos: Vec2,
    pub rotation: f32,
    pub clicks: u32,
    pub animator: Animator,
}

impl Rock {
    pub fn new(assets: &GlobalAssets, id: u32, name: String, pos: Vec2, rotation: f32) -> Self {
        let mut animator = Animator::new(assets.rock_sprite.clone(), vec2(16.0, 16.0));

        animator.add_animation("mining", 3, 10, 0.0, false);

        Self {
            id,
            name,
            pos,
            rotation,
            clicks: 0,
            animator,
        }
    }
}

impl Interactable for Rock {
    fn interact(&mut self) -> Result<GameStateAction, GameError> {
        Ok(GameStateAction::SpawnOverlay(MineRock::new(self.id)?))
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_sprite_size(&self) -> Vec2 {
        self.animator.frame_size
    }

    fn get_animator(&self) -> &Animator {
        &self.animator
    }

    fn update_animation(&mut self, _delta_time: f32) -> Result<(), GameError> {
        // convert clicks to frame, where there are 8 frames and 16 clicks
        let frame = if self.clicks > 0 {
            (self.clicks / 2) % 8
        } else {
            0
        };

        self.animator.current_frame = frame as usize;

        Ok(())
    }

    fn draw(&self, _data: &GameData) -> Result<(), GameError> {
        self.animator.draw(self.pos, Some(self.rotation), None);
        Ok(())
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
            animator: self.animator.clone(),
        })
    }

    fn is_mouse_over(&self, _data: &GameData) -> bool {
        let mouse_pos = vec2(mouse_position().0, mouse_position().1);
        let rect = Rect::new(self.pos.x, self.pos.y, self.get_sprite_size().x, self.get_sprite_size().y);
        rect.contains(mouse_pos)
    }

    fn distance_from_player(&self, data: &GameData) -> f32 {
        let player = &data.world.player;
        let player_pos = vec2(player.pos.x + player.sprite.width() / 2.0, player.pos.y + player.sprite.height() / 2.0);
        let sprite = self.get_sprite_size();
        let interactable_pos = vec2(self.pos.x + sprite.x / 2.0, self.pos.y + sprite.y / 2.0);
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