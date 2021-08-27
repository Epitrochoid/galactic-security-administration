use macroquad::prelude::*;

mod player;
mod item;
mod bag;
mod scene;
mod scenes;
mod assets;

use crate::assets::manager::AssetManager;
use crate::scenes::main_menu::MainMenu;
use crate::scenes::level::Level;
use crate::scene::SceneManager;

#[macroquad::main("Galactic Security Administration")]
async fn main() {
    let mut asset_manager = AssetManager::new();
    asset_manager.load_assets().await;

    let first_level = Level::new("This is the intro".to_string());
    let mut scene_manager = SceneManager {
        current_scene: Box::new(MainMenu::new(first_level))
    };
    loop {
        clear_background(BLACK);

        scene_manager.update();
        scene_manager.draw(&asset_manager);

        next_frame().await
    }
}
