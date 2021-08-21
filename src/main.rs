use macroquad::prelude::*;

mod player;
mod item;
mod bag;
mod scene;
mod scenes;

use crate::scenes::main_menu::MainMenu;
use crate::scene::SceneManager;

#[macroquad::main("Galactic Security Administration")]
async fn main() {
    let scene_manager = SceneManager {current_scene: Box::new(MainMenu{})};
    loop {
        clear_background(BLACK);

        scene_manager.draw();

        next_frame().await
    }
}
