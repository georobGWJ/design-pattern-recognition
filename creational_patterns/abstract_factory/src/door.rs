use map_site::MapSite;

pub struct Door {
    room1: u32,
    room2: u32,
    is_open: bool,
}

impl Door {
    // Constructor
    pub fn new(room1: u32, room2: u32) -> Door {
        Door {
            room1: room1,
            room2: room2,
            is_open: false,
        }
    }

    pub fn other_side_of_door(&self) -> u32 {
        self.room2
    }
}

impl MapSite for Door {
    fn enter(&self) {
        println!("It's a wooden door.");
    }
}
