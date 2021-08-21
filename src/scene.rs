pub enum Message {
    None,
    NextScene(Box<Scene>),
}

pub trait Scene {
    fn update(&mut self) -> Message;
    fn draw(&self) -> ();
}

pub struct SceneManager {
    current_scene: Box<Scene>,
}

impl SceneManager {
    fn update(&mut self) {
        match self.current_scene.update() {
            Message::None => (),
            Message::NextScene(scene) => self.current_scene = scene,
        };
    }

    fn draw(&self) {
        self.current_scene.draw();
    }
}
