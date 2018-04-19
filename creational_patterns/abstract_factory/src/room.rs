use map_site::MapSite;
use wall::Wall;
use door::Door;

//#[derive(Debug, Clone, Copy)]
enum RoomSide {
    wall(Wall),
    door(Door),
}

pub struct Room {
    room_number: u32,
    sides: [RoomSide; 4],  // TO-DO: How to define array or vector for MapSite objects?
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
