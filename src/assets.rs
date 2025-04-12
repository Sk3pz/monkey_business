use macroquad::prelude::*;

const FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/PressStart2P-Regular.ttf");
const PLAYER_SHEET_BYTES: &[u8] = include_bytes!("../assets/sprites/monke2.png");
const ROCK_SHEET_BYTES: &[u8] = include_bytes!("../assets/sprites/Rocks floor and decor.png");

#[derive(Clone)]
pub struct GlobalAssets {
    pub font: Font,
    pub rock_sprite: Texture2D,
    pub player_sprite: Texture2D,
}

impl GlobalAssets {

    pub async fn load() -> Result<Self, String> {
        // let Ok(font) = load_ttf_font("./assets/fonts/PressStart2P-Regular.ttf").await else {
        //     return Err("Failed to load font".to_string());
        // };
        let Ok(font) = load_ttf_font_from_bytes(FONT_BYTES) else {
            return Err("Failed to load font".to_string());
        };

        //let rock = load_texture("assets/sprites/Rocks floor and decor.png").await;
        // if let Err(e) = rock {
        //     return Err(format!("Failed to load rock texture files: {}", e));
        // }
        // let rock_sprite = rock.unwrap();
        let rock_sprite = Texture2D::from_file_with_format(ROCK_SHEET_BYTES, Some(ImageFormat::Png));
        rock_sprite.set_filter(FilterMode::Nearest);

        let player_sprite = Texture2D::from_file_with_format(PLAYER_SHEET_BYTES, Some(ImageFormat::Png));
        player_sprite.set_filter(FilterMode::Nearest);

        Ok(Self {
            font,
            rock_sprite,
            player_sprite,
        })
    }

}