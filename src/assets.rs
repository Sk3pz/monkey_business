use macroquad::prelude::*;

#[derive(Clone)]
pub struct GlobalAssets {
    pub font: Font,
    pub rock_sprites: Vec<Texture2D>,
}

impl GlobalAssets {

    pub async fn load() -> Result<Self, String> {
        let Ok(font) = load_ttf_font("./assets/fonts/PressStart2P-Regular.ttf").await else {
            return Err("Failed to load font".to_string());
        };

        let mut rock_sprites = Vec::new();
        let rock = load_texture("assets/sprites/example_rock.png").await;
        if let Err(e) = rock {
            return Err(format!("Failed to load rock texture files: {}", e));
        }
        let rock_sprite = rock.unwrap();
        rock_sprite.set_filter(FilterMode::Nearest);
        rock_sprites.push(rock_sprite);

        Ok(Self {
            font,
            rock_sprites
        })
    }

}