use macroquad::prelude::*;
use std::collections::HashMap;

pub struct TextureManager {
    textures: HashMap<String, Texture2D>,
}

const TEXTURE_MAP: &[(&str, &str)] = &[
    ("chest", "assets/chest.png"),
    ("enemy_chest", "assets/enemy_chest.png"),
    ("gold", "assets/gold.png"),
    ("tile_decorations", "assets/tile_decorations.png"),
    ("player", "assets/player_spritesheet.png"),
    ("enemy", "assets/enemy_spritesheet.png"),
];

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub async fn load_all_textures(&mut self) {
        for (name, path) in TEXTURE_MAP {
            let texture = load_texture(path).await.unwrap();
            self.textures.insert(name.to_string(), texture);
        }
        build_textures_atlas();
    }

    pub fn get(&self, texture_name: &str) -> Option<&Texture2D> {
        self.textures.get(texture_name)
    }
}
