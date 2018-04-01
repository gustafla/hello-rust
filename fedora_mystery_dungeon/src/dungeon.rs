use std::vec::Vec;

pub struct Dungeon {
    floors: i32,
    items: Vec<Item>, // TODO for floor ranges
    monsters: Vec<Monster>, // todo for floor ranges
}
