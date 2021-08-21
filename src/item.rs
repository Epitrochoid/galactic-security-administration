use macroquad::prelude::*;

pub enum ItemType {
    Blaster,
    Sock,
}

pub struct Item {
    item_type: ItemType,
    dimensions: Vec2,
}
