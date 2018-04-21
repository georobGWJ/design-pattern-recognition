use map_site::MapSite;
use super::Direction;

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

    pub fn get_side(&self, direction: Direction) -> &Box<MapSite> {
        // North = 0, East = 1, South = 2, West = 3
        let dir = direction.value();
        // let dir = match direction {
        //     Direction::North => 0 as usize,
        //     Direction::East =>  1 as usize,
        //     Direction::South => 2 as usize,
        //     Direction::West =>  3 as usize,
        // };

        &self.sides[dir]
    }

    pub fn set_side(&mut self, direction: Direction, side: Box<MapSite>) {
        // North = 0, East = 1, South = 2, West = 3
        let dir = direction.value();
        self.sides[dir] = side;
    }
}

impl MapSite for Room {
    fn enter(&self) {
        println!("What a pretty room.");
    }
}
