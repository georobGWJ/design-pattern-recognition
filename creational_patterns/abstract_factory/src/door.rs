use map_site::MapSite;
use Room;

struct Door {
    room1: Room,
    room2: Room,
    is_open: bool,
}

impl Door {
    // Constructor
    fn door(room1: &Room, room2: &Room) -> Door {
        unimplemented!();
    }
}

impl MapSite for Door {
    fn enter(&self) {
        println!("It's a wooden door.");
    }
}
