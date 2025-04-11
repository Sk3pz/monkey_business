use macroquad::prelude::*;

#[derive(Clone)]
pub struct GlobalAssets {
    pub font: Font,
    pub rock_sprite: Texture2D,
}

impl GlobalAssets {

    pub async fn load() -> Result<Self, String> {
        let Ok(font) = load_ttf_font("./assets/fonts/PressStart2P-Regular.ttf").await else {
            return Err("Failed to load font".to_string());
        };

        let rock = load_texture("assets/sprites/Rocks floor and decor.png").await;
        if let Err(e) = rock {
            return Err(format!("Failed to load rock texture files: {}", e));
        }
        let rock_sprite = rock.unwrap();
        rock_sprite.set_filter(FilterMode::Nearest);

        Ok(Self {
            font,
            rock_sprite
        })
    }

}