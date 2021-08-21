use macroquad::prelude::*;

use crate::scene::{Scene, Message};

pub struct MainMenu{}

impl Scene for MainMenu {
    fn update(&mut self) -> Message {
        Message::None
    }

    fn draw(&self) -> () {
        draw_text("Galactic Security Administration", 100., 100., 30., WHITE);
    }
}
