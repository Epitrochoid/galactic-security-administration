use std::collections::HashMap;
use macroquad::prelude::*;

pub struct Asset2d {
    pub key: String,
    pub loaded_texture: Texture2D,
}

impl Asset2d {
    pub fn new(key: String, texture: Texture2D) -> Asset2d {
        Asset2d {
            key: key,
            loaded_texture: texture
        }
    }
}

pub struct AssetManager {
    pub assets_map: HashMap<String, Asset2d>,
}

impl AssetManager {
    pub fn new() -> AssetManager {
        AssetManager {
            assets_map: HashMap::new(),
        }
    }

    pub async fn load_assets(&mut self) {
        let file_path = "static/assets/textures/wave.png";
        let texture = load_texture(file_path).await.unwrap();
        let key = "wave";
        self.assets_map.insert(key.to_string(), Asset2d::new(key.to_string(), texture));
    }
}
