use std::f32::consts::PI;
use macroquad::input::mouse_position;
use macroquad::math::{vec2, Vec2};
use macroquad::window::{screen_height, screen_width};
use crate::world::interactable::Interactable;

pub const PLAYER_SPEED: f32 = 5.0;
const PLAYER_SCALE: (f32, f32) = (16.0, 16.0);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlayerFacing {
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

#[derive(Clone)]
pub struct Player {
    pub pos: Vec2,
    pub rotation: f32,
    pub sprinting: bool,
    pub facing: PlayerFacing,
}

impl Player {
    
    pub async fn new() -> Result<Self, String> {
        Ok(Self {
            pos: vec2(0.0, 0.0),
            rotation: 0.0,
            sprinting: false,
            facing: PlayerFacing::UpRight,
        })
    }

    pub fn apply_movement(&mut self, player_sprite_size: Vec2, mut movement: Vec2, interactables: &Vec<Box<dyn Interactable>>, delta_time: u128) {
        if delta_time == 0 {
            return;
        }
        if movement.length() > 0.0 {
            movement = movement.normalize() * (PLAYER_SPEED / delta_time as f32);
        }

        // sprinting
        if self.sprinting {
            movement *= 2.0;
        }

        // add collisions with interactables
        for interactable in interactables {
            let player_size = vec2(player_sprite_size.x, player_sprite_size.y);
            let player_pos = vec2(self.pos.x - player_size.x / 2.0, self.pos.y - player_size.y / 2.0);

            let interactable_sprite = interactable.get_sprite_size();
            let interactable_size = vec2(interactable_sprite.x, interactable_sprite.y);
            let ipos = interactable.get_pos();
            let interactable_pos = vec2(
                ipos.x - interactable_size.x / 2.0,
                ipos.y - interactable_size.y / 2.0,
            );

            // AABB collision check
            if player_pos.x < interactable_pos.x + interactable_size.x
                && player_pos.x + player_size.x > interactable_pos.x
                && player_pos.y < interactable_pos.y + interactable_size.y
                && player_pos.y + player_size.y > interactable_pos.y
            {
                // Calculate overlap (penetration) in each direction
                let left_penetration = (player_pos.x + player_size.x) - interactable_pos.x;
                let right_penetration = (interactable_pos.x + interactable_size.x) - player_pos.x;
                let top_penetration = (player_pos.y + player_size.y) - interactable_pos.y;
                let bottom_penetration = (interactable_pos.y + interactable_size.y) - player_pos.y;

                let min_penetration = left_penetration
                    .min(right_penetration)
                    .min(top_penetration)
                    .min(bottom_penetration);

                // Stop movement in the direction of the smallest overlap
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