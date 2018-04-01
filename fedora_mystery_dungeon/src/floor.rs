use std::vec::Vec;

struct Floor {
    rooms: Vec<Room>,
    tunnels: Vec<Tunnel>,
    items: Vec<Item>, // TODO for floor ranges
    monsters: Vec<Monster>, // todo for floor ranges
}
