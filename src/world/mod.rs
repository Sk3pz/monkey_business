use macroquad::math::vec2;
use macroquad::prelude::load_texture;
use crate::gamestate::{GameStateAction, GameStateError};
use crate::{debug, player};
use crate::ui::tooltip::ToolTipCard;
use crate::world::interactable::Interactable;

pub mod interactable;

pub async fn craft_example_rock() -> Result<Interactable, String> {
    // rock texture
    let rock = load_texture("assets/sprites/example_rock.png").await;
    if let Err(e) = rock {
        return Err(format!("Failed to load texture files: {}", e));
    }
    let rock = rock.unwrap();

    Ok(Interactable::new(
        "Rock Test".to_string(),
        ToolTipCard::new(format!("{}Rock Test", better_term::Color::BrightWhite),
        vec![format!("{}Press {}e{} to interact.", better_term::Color::White,
                     better_term::Color::BrightYellow, better_term::Color::White)]),
        vec2(100.0, 100.0),
        rock,
        0.0,
        |player: &mut player::Player| {



            Ok(GameStateAction::NoOp)
        }
    ))
}