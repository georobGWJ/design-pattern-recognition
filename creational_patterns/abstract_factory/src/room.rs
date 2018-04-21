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

    pub fn get_side(&self, direction: usize) -> &Box<MapSite> {
        // North = 0, East = 1, South = 2, West = 3
        &self.sides[direction]
    }
}

impl MapSite for Room {
    fn enter(&self) {
        println!("What a pretty room.");
    }
}
