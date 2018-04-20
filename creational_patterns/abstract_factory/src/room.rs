use map_site::MapSite;

pub struct Room {
    room_number: u32,
    sides: Vec<Box<MapSite>>,
}

impl Room {
    // Constructor
    pub fn new(number: u32, sides: Vec<Box<MapSite>>) -> Room {
        Room {
            room_number: number,
            sides: sides,
        }
    }
}

impl MapSite for Room {
    fn enter(&self) {
        println!("What a pretty room.");
    }
}
