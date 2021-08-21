use macroquad::prelude::*;

use crate::scene::{Scene, Message};

pub struct GameOver{}

impl Scene for GameOver {
    fn update(&mut self) -> Message {
        Message::None
    }

    fn draw(&mut self) -> () {
        draw_text("You Lose", 100., 100., 30., WHITE);
    }
}
