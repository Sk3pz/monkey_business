use macroquad::color::WHITE;
use macroquad::math::vec2;
use macroquad::prelude::{draw_texture_ex, screen_height, screen_width, DrawTextureParams};
use macroquad::rand::gen_range;
use crate::assets::GlobalAssets;
use crate::controls::Action;
use crate::error::GameError;
use crate::gamedata::GameData;
use crate::ui::tooltip::{tooltip, ToolTipCard};
use crate::util::{get_sprite_scale, scale_position};
use crate::world::rock::Rock;
use crate::world::interactable::Interactable;
use crate::world::player::{Player, PlayerFacing};

pub mod interactable;
pub mod rock;
pub mod player;

pub struct World {
    pub player: Player,
    pub interactables: Vec<Box<dyn Interactable>>,
}

impl World {
    pub async fn new(assets: &GlobalAssets) -> Result<Self, String> {
        let player = Player::new().await;
        if let Err(e) = player {
            return Err(format!("Failed to initialize player: {}", e));
        }
        let player = player?;

        let mut interactables: Vec<Box<dyn Interactable>> = Vec::new();

        for x in 0..5 {
            let rock = Rock::new(assets, x, "Rock Pile".to_string(), vec2(gen_range(20.0, screen_width() - 20.0), gen_range(20.0, screen_height() - 20.0)), gen_range(0.0, 360.0));

            interactables.push(Box::new(rock));
        }

        Ok(Self {
            player,
            interactables,
        })
    }

    pub fn get_interactable_by_id(&self, id: u32) -> Option<&Box<dyn Interactable>> {
        self.interactables.iter().find(|i| i.get_id() == id)
    }

    pub fn get_mut_interactable_by_id(&mut self, id: u32) -> Option<&mut Box<dyn Interactable>> {
        self.interactables.iter_mut().find(|i| i.get_id() == id)
    }

    pub fn break_interactable(&mut self, id: u32) -> Result<(), String> {
        if let Some(_) = self.get_mut_interactable_by_id(id) {
            // remove the interactable from the world
            self.interactables.retain(|i| i.get_id() != id);
            Ok(())
        } else {
            Err(format!("Failed to find interactable with id: {}", id))
        }
    }

    pub fn add_interactable(&mut self, interactable: Box<dyn Interactable>) {
        self.interactables.push(interactable);
    }

    pub fn draw_player(&self, data: &GameData) {
        let postion_scale = scale_position(self.player.pos);

        draw_texture_ex(
            &data.assets.player_sprite,
            postion_scale.x, postion_scale.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(get_sprite_scale()),
                //dest_size: Some(vec2(32.0, 32.0)),
                rotation: self.player.rotation,
                flip_x: self.player.facing == PlayerFacing::UpLeft || self.player.facing == PlayerFacing::DownLeft,
                flip_y: false, // never true because of isometric projection
                source: None, // todo: use for animation sprite sheets
                ..Default::default()
            }
        );
    }

    pub fn is_click_on_interactable(&self, data: &GameData) -> Option<u32> {
        for interactable in &self.interactables {
            if interactable.is_mouse_over(data) && interactable.distance_from_player(data) <= 100.0 {
                return Some(interactable.get_id());
            }
        }
        None
    }

    pub fn update_interactables(&mut self, delta_time: f32) -> Result<(), GameError> {
        for interactable in &mut self.interactables {
            interactable.update_animation(delta_time)?;
        }
        Ok(())
    }

    pub fn draw_interactables(&self, data: &GameData) -> Result<(), GameError> {
        for interactable in &self.interactables {
            interactable.draw(data)?;
        }
        Ok(())
    }

    pub fn handle_tooltips(&self, data: &GameData) {
        // if the mouse is on an interactable, give a tooltip
        for interactable in &self.interactables {
            if interactable.is_mouse_over(data) {
                if interactable.distance_from_player(data) <= 100.0 {
                    let interact_btn = data.control_handler.get_binding(&Action::Interact).unwrap();
                    // let clicks = match interactable.get_attribute("clicks").unwrap_or(InteractableAttribute::UInt(0)) {
                    //     InteractableAttribute::UInt(i) => i,
                    //     _ => 0
                    // };
                    let card = ToolTipCard {
                        title: interactable.get_name(),
                        lines: vec![format!("{}Press {}{}{} to interact.", better_term::Color::White,
                                            better_term::Color::BrightYellow, interact_btn, better_term::Color::White),
                                    //format!("{}Clicks: {}{}", better_term::Color::White, better_term::Color::BrightYellow, clicks)
                        ],
                    };
                    tooltip(card, &data.assets);
                } else {
                    let card = ToolTipCard {
                        title: interactable.get_name(),
                        lines: vec![format!("{}Get closer to interact!", better_term::Color::White)],
                    };
                    tooltip(card, &data.assets);
                }
            }
        }
    }

}