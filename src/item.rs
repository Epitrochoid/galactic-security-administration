use macroquad::prelude::*;

use crate::assets::manager::AssetManager;

pub struct Item {
    dimensions: Vec2,
    pub texture_key: &'static str,
}

impl Item {
    pub fn load_items(asset_manager: &AssetManager) -> Vec<Item> {
        vec![ Item {
            dimensions: Vec2::new(128., 128.),
            texture_key: "blaster",
        }]
    }
}
