use map_site::MapSite;
use room::Room;

pub struct Door {
    room1: Room,
    room2: Room,
    is_open: bool,
}

impl Door {
    // Constructor
    pub fn new(room1: &Room, room2: &Room) -> Door {
        unimplemented!();
    }

    pub fn other_side_of_door(&self) -> Room {
        unimplemented!();
    }
}

impl MapSite for Door {
    fn enter(&self) {
        println!("It's a wooden door.");
    }
}
