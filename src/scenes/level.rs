use macroquad::prelude::*;
use macroquad::ui::{
    root_ui
};

use crate::scene::{Scene, Message};

pub struct Level {
    pub intro_text: String,
}

pub struct LevelIntro {
    pub intro_text: String,
    pub main_level: Option<Box<Scene>>,
    pub start: bool,
}

impl Scene for LevelIntro {
    fn update(&mut self) -> Message {
        match self.start {
            true => Message::NextScene(self.main_level.take().expect("LevelIntro to be dropped")),
            false => Message::None,
        }
    }

    fn draw(&mut self) -> () {
        draw_text(&self.intro_text, 200., 100., 40., WHITE);
        if root_ui().button(vec2(200., 200.), "Start Shift!") {
            self.start = true;
        }
    }
}

struct LevelBody {
}

impl Scene for LevelBody {
    fn update(&mut self) -> Message {
        Message::None
    }

    fn draw(&mut self) -> () {
        draw_rectangle(100., 90., 520., 300., WHITE);
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
