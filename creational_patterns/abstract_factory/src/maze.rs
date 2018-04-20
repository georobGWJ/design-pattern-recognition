use room::Room;

pub struct Maze {
    current_room: u32,
    rooms: Vec<Box<Room>>,
}

impl Maze {
    // Constructor
    pub fn new(start: u32, rooms: Vec<Box<Room>>) -> Maze {
        Maze {
            current_room: start,
            rooms: rooms,
        }
    }

    pub fn get_current_room(&self) -> u32 {
        self.current_room
    }

    pub fn set_current_room(&mut self, room: u32) {
        self.current_room = room;
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(Box::new(room));
    }
}
