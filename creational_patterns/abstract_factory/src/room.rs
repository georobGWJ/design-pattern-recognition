use map_site::MapSite;

struct Room {
    room_number: u32,
    sides: vec!<map_site>,
}

impl Room {
    // Constructor
    fn room(number: u32) -> Room {
        unimplemented!();
    }
}

impl MapSite for Room {
    fn enter(&self) {
        println!("What a pretty room.");
    }
}
