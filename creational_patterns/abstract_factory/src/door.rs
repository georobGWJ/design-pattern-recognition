use map_site::MapSite;
use room::Room;

//#[derive(Debug)]
pub struct Door {
    room1: u32,
    room2: u32,
    is_open: bool,
}

impl Door {
    // Constructor
    pub fn new(room1: u32, room2: u32) -> Door {
        Door {
            room1: room1,  // current room (is this needed?)
            room2: room2,  // room you enter when entering this door
            is_open: false,
        }
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
