use std::f32::consts::PI;
use macroquad::input::mouse_position;
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{load_texture, Texture2D};
use macroquad::window::{screen_height, screen_width};
use crate::debug;
use crate::world::interactable::Interactable;

pub const PLAYER_SPEED: f32 = 5.0;
const PLAYER_SCALE: (f32, f32) = (32.0, 32.0);
const COLLISION_PADDING: f32 = 10.0; // padding for collisions

#[derive(Clone)]
pub struct Player {
    pub pos: Vec2,
    pub sprite: Texture2D,
    pub rotation: f32
}

impl Player {
    
    pub async fn new() -> Result<Self, String> {
        
        // player texture
        let player = load_texture("assets/sprites/monke.png").await;
        if let Err(e) = player {
            return Err(format!("Failed to load texture files: {}", e));
        }
        let player = player.unwrap();
        
        Ok(Self {
            pos: vec2(0.0, 0.0),
            sprite: player,
            rotation: 0.0,
        })
    }

    pub fn apply_movement(&mut self, mut movement: Vec2, interactables: &Vec<Interactable>, delta_time: u128) {
        if delta_time == 0 {
            return;
        }
        if movement.length() > 0.0 {
            movement = movement.normalize() * (PLAYER_SPEED / delta_time as f32);
        }

        // add collisions with interactables
        for interactable in interactables {
            let interactable_pos = vec2(interactable.pos.x + COLLISION_PADDING, interactable.pos.y + COLLISION_PADDING);
            let interactable_size = vec2(interactable.sprite.width() - COLLISION_PADDING * 2.0, interactable.sprite.height() - COLLISION_PADDING * 2.0);

            if self.pos.x < interactable_pos.x + interactable_size.x && self.pos.x + PLAYER_SCALE.0 > interactable_pos.x &&
                self.pos.y < interactable_pos.y + interactable_size.y && self.pos.y + PLAYER_SCALE.1 > interactable_pos.y {
                // stop the player from moving into the interactable while allowing the player to move in all other directions
                // Calculate penetration depths
                let left_penetration = (self.pos.x + PLAYER_SCALE.0) - interactable_pos.x;
                let right_penetration = (interactable_pos.x + interactable_size.x) - self.pos.x;
                let top_penetration = (self.pos.y + PLAYER_SCALE.1) - interactable_pos.y;
                let bottom_penetration = (interactable_pos.y + interactable_size.y) - self.pos.y;

                // Find the minimum penetration direction
                let min_penetration = left_penetration.min(right_penetration)
                    .min(top_penetration)
                    .min(bottom_penetration);

                // Resolve collision based on minimum penetration
                if min_penetration == left_penetration && movement.x > 0.0 {
                    movement.x = 0.0;
                } else if min_penetration == right_penetration && movement.x < 0.0 {
                    movement.x = 0.0;
                }
                if min_penetration == top_penetration && movement.y > 0.0 {
                    movement.y = 0.0;
                } else if min_penetration == bottom_penetration && movement.y < 0.0 {
                    movement.y = 0.0;
                }

                debug!("Collision: {}", interactable);
            }
        }

        self.pos += movement;

        // hard cap position at window bounds
        if self.pos.x < 0.0 {
            self.pos.x = 0.0;
        } else if self.pos.x > screen_width() - PLAYER_SCALE.0 {
            self.pos.x = screen_width() - PLAYER_SCALE.0;
        }
        if self.pos.y < 0.0 {
            self.pos.y = 0.0;
        } else if self.pos.y > screen_height() - PLAYER_SCALE.0 {
            self.pos.y = screen_height() - PLAYER_SCALE.0;
        }

        // teleport the player to 0,0 if they go out of bounds
        if self.pos.x < -PLAYER_SCALE.0 || self.pos.x > screen_width() + PLAYER_SCALE.0 ||
            self.pos.y < -PLAYER_SCALE.0 || self.pos.y > screen_height() + PLAYER_SCALE.0 {
            self.pos = vec2(0.0, 0.0);
        }

    }

    pub fn look_towards_mouse(&mut self) {
        let mouse_pos = mouse_position();

        let pos = vec2(self.pos.x + PLAYER_SCALE.0 / 2.0, self.pos.y + PLAYER_SCALE.1 / 2.0);

        let dx = mouse_pos.0 - pos.x;
        let dy = mouse_pos.1 - pos.y;
        
        self.rotation = dy.atan2(dx) + (PI / 2.0);
    }

    pub fn is_on_mouse(&self) -> bool {
        let mouse_pos = mouse_position();
        let pos = vec2(self.pos.x + PLAYER_SCALE.0 / 2.0, self.pos.y + PLAYER_SCALE.1 / 2.0);

        let dx = mouse_pos.0 - pos.x;
        let dy = mouse_pos.1 - pos.y;

        dx.abs() < PLAYER_SCALE.0 / 2.0 && dy.abs() < PLAYER_SCALE.1 / 2.0
    }
    
}