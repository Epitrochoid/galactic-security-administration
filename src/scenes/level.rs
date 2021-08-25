use macroquad::prelude::*;
use macroquad::ui::{
    root_ui
};

use crate::assets::manager::AssetManager;
use crate::scene::{Scene, Message};

pub struct Level {
    pub intro_text: String,
}

pub struct LevelIntro {
    pub intro_text: String,
    pub main_level: Option<Box<dyn Scene>>,
    pub start: bool,
}

impl Scene for LevelIntro {
    fn update(&mut self) -> Message {
        match self.start {
            true => Message::NextScene(self.main_level.take().expect("LevelIntro to be dropped")),
            false => Message::None,
        }
    }

    fn draw(&mut self, asset_manager: &AssetManager) -> () {
        if root_ui().button(vec2(200., 200.), "Start Shift!") {
            self.start = true;
        }

        draw_text(&self.intro_text, 200., 100., 40., WHITE);
    }
}

struct LevelBody {
}

impl Scene for LevelBody {
    fn update(&mut self) -> Message {
        Message::None
    }

    fn draw(&mut self, asset_manager: &AssetManager) -> () {
        draw_rectangle(100., 90., 520., 300., Color::new(0.5, 0.5, 0.7, 1.0));
        let asset_option = asset_manager.assets_map.get(&"wave".to_string());
        if (asset_option.is_some()) {
            let asset = asset_option.unwrap();
            draw_texture(asset.loaded_texture, 100., 90., WHITE);
        }
    }
}

impl Level {
    pub fn new(intro_text: String) -> Box<LevelIntro> {
        let level_body = Box::new(LevelBody{});

        Box::new(LevelIntro {
            intro_text: intro_text,
            main_level: Some(level_body),
            start: false,
        })
    }
}
