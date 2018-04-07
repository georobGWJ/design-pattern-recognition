use map_site::MapSite;

pub struct Room {
    room_number: u32,
    // sides: [MapSite; 4],  // TO-DO: How to define array or vector for MapSite objects?
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
