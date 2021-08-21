use macroquad::prelude::*;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Ui,
};

use crate::scene::{Scene, Message};
use crate::scenes::game_over::GameOver;

pub struct MainMenu {
    pub game_started: bool,
}

impl Scene for MainMenu {
    fn update(&mut self) -> Message {
        match self.game_started {
            true => Message::NextScene(Box::new(GameOver{})),
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
