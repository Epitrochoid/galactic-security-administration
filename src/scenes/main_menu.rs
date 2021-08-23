use macroquad::prelude::*;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Ui,
};

use crate::scene::{Scene, Message};

pub struct MainMenu {
    pub game_started: bool,
    pub first_level: Option<Box<Scene>>,
}

impl MainMenu {
    pub fn new(first_level: Box<Scene>) -> Self {
        MainMenu {
            game_started: false,
            first_level: Some(first_level),
        }
    }
}

impl Scene for MainMenu {
    fn update(&mut self) -> Message {
        match self.game_started {
            true => Message::NextScene(self.first_level.take().expect("MainMenu to be dropped")),
            false => Message::None,
        }
    }

    fn draw(&mut self) -> () {
        draw_text("Galactic Security Administration", 100., 100., 30., WHITE);

        if root_ui().button(vec2(100., 150.), "Start Game") {
            println!("start game pressed");
            self.game_started = true;
        }
    }
}
