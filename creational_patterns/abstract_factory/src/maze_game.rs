use door::*;
use wall::*;
use room::*;
use maze::*;
use map_site::*;

pub struct MazeGame {
    maze: Maze,
}

impl MazeGame {
    // Hard-coded Constructor for a 2 room Maze
    fn new() -> MazeGame {

        let door = Door::new( 1, 2 );
        let room1_sides : Vec<Box<MapSite>> = vec![
            Box::new( Wall {} ),
            Box::new( door ),
            Box::new( Wall {} ),
            Box::new( Wall {} )];

        let door = Door::new( 1, 2 );
        let room2_sides : Vec<Box<MapSite>> = vec![
            Box::new( Wall {} ),
            Box::new( Wall {} ),
            Box::new( Wall {} ),
            Box::new( door )];

        let mut room1 = Room::new(1, room1_sides);
        let mut room2 = Room::new(2, room2_sides);

        let rooms: Vec<Box<Room>> = vec![ Box::new(room1), Box::new(room2) ];

        let aMaze = Maze::new(1, rooms);

        MazeGame { maze: aMaze }
    }
}
