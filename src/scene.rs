use crate::assets::manager::AssetManager;


pub enum Message {
    None,
    NextScene(Box<dyn Scene>),
}

pub trait Scene {
    fn update(&mut self) -> Message;
    fn draw(&mut self, asset_manager: &AssetManager) -> ();
}

pub struct SceneManager {
    pub current_scene: Box<dyn Scene>,
}

impl SceneManager {
    pub fn update(&mut self) {
        match self.current_scene.update() {
            Message::None => (),
            Message::NextScene(scene) => self.current_scene = scene,
        };
    }

    pub fn draw(&mut self, asset_manager: &AssetManager) {
        self.current_scene.draw(asset_manager);
    }
}
