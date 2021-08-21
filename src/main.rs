use macroquad::prelude::*;

mod player;
mod item;
mod bag;
mod scene;

#[macroquad::main("Galactic Security Administration")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_text("Galactic Security Administration", 100., 100., 30., WHITE);

        next_frame().await
    }
}
