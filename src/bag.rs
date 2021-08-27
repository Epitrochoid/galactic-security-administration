use rand::seq::SliceRandom;

use crate::item::Item;

pub struct Bag<'a> {
    items: &'a Vec<&'a Item>,
    contains_contraband: bool,
}

impl Bag<'_> {
    pub fn new(available_items: Vec<Item>, contraband_type: &str) -> Self {
        let items = available_items
            .choose_multiple(&mut rand::thread_rng(), 10)
            .collect();
        let contains_contraband = available_items.iter().any(|&item| item.texture_key == contraband_type);

        Bag {
            items: &items,
            contains_contraband,
        }
    }
}
