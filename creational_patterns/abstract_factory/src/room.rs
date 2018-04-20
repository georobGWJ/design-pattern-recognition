use map_site::MapSite;
use wall::Wall;
use door::Door;

pub struct Room {
    room_number: u32,
    sides: Vec<Box<MapSite>>,
}

impl Room {
    // Constructor
    pub fn room(number: u32) -> Room {
        unimplemented!();
    }
}

impl MapSite for Room {
    fn enter(&self) {
        println!("What a pretty room.");
    }
}
